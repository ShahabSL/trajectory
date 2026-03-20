use crate::auth::ClientAccessKey;
use crate::protocol::{
    build_empty_response, build_response, decode_query_request, parse_query, FLAG_DATA,
    FLAG_DOWNLINK, FLAG_FIN, DNS_MAX_PAYLOAD, MAX_RESPONSE_PAYLOAD, RESPONSE_CHUNK_SIZE,
};
use anyhow::{Context, Result};
use hickory_proto::op::ResponseCode;
use hickory_proto::rr::RecordType;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::{mpsc, watch, Mutex};

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub bind: SocketAddr,
    pub domain: String,
    pub target: SocketAddr,
    pub authorized_clients: Arc<HashMap<u32, ClientAccessKey>>,
}

struct Session {
    writer: tokio::net::tcp::OwnedWriteHalf,
    client_key: ClientAccessKey,
    next_uplink_seq: u32,
    pending_uplink: BTreeMap<u32, Vec<u8>>,
    next_down_seq: u32,
    down_queue: VecDeque<u8>,
    down_chunks: BTreeMap<u32, Vec<u8>>,
    last_seen: Instant,
    client_closed: bool,
    remote_closed: bool,
}

struct Shared {
    sessions: Mutex<HashMap<u64, Arc<Mutex<Session>>>>,
}

pub async fn run(config: ServerConfig) -> Result<()> {
    let (_shutdown_tx, shutdown_rx) = watch::channel(false);
    run_until(config, shutdown_rx).await
}

pub async fn run_until(config: ServerConfig, mut shutdown_rx: watch::Receiver<bool>) -> Result<()> {
    let socket = Arc::new(UdpSocket::bind(config.bind).await?);
    let shared = Arc::new(Shared {
        sessions: Mutex::new(HashMap::new()),
    });
    let janitor = shared.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(30)).await;
            let mut sessions = janitor.sessions.lock().await;
            sessions.retain(|_, session| {
                if let Ok(guard) = session.try_lock() {
                    guard.last_seen.elapsed() < Duration::from_secs(120)
                } else {
                    true
                }
            });
        }
    });

    let mut buf = vec![0u8; 2048];
    loop {
        let recv = tokio::select! {
            changed = shutdown_rx.changed() => {
                changed.ok();
                return Ok(());
            }
            result = socket.recv_from(&mut buf) => result,
        };
        let (len, peer) = recv?;
        let packet = buf[..len].to_vec();
        let socket = socket.clone();
        let shared = shared.clone();
        let domain = config.domain.clone();
        let target = config.target;
        let authorized_clients = config.authorized_clients.clone();
        tokio::spawn(async move {
            if let Err(error) = handle_query(
                socket,
                shared,
                &domain,
                target,
                peer,
                &packet,
                authorized_clients,
            )
            .await
            {
                eprintln!("server query error: {error:#}");
            }
        });
    }
}

async fn handle_query(
    socket: Arc<UdpSocket>,
    shared: Arc<Shared>,
    domain: &str,
    target: SocketAddr,
    peer: SocketAddr,
    packet: &[u8],
    authorized_clients: Arc<HashMap<u32, ClientAccessKey>>,
) -> Result<()> {
    let query = match parse_query(packet) {
        Ok(query) => query,
        Err(_) => return Ok(()),
    };
    if query.qtype != u16::from(RecordType::TXT) {
        let wire = build_empty_response(&query, ResponseCode::NoError)?;
        socket.send_to(&wire, peer).await?;
        return Ok(());
    }
    let request = match decode_query_request(&query, domain) {
        Ok(request) => request,
        Err(_) => {
            let wire = build_empty_response(&query, ResponseCode::NoError)?;
            socket.send_to(&wire, peer).await?;
            return Ok(());
        }
    };
    let Some(access_key) = authorized_clients.get(&request.client_id).cloned() else {
        let wire = build_empty_response(&query, ResponseCode::NoError)?;
        socket.send_to(&wire, peer).await?;
        return Ok(());
    };
    if !request.verify(&access_key).unwrap_or(false) {
        let wire = build_empty_response(&query, ResponseCode::NoError)?;
        socket.send_to(&wire, peer).await?;
        return Ok(());
    }
    let session = get_or_create_session(shared, request.session_id, target, access_key.clone()).await?;
    let response = {
        let mut session = session.lock().await;
        if session.client_key.client_id != access_key.client_id {
            let wire = build_empty_response(&query, ResponseCode::NoError)?;
            socket.send_to(&wire, peer).await?;
            return Ok(());
        }
        session.last_seen = Instant::now();
        apply_down_ack(&mut session, request.down_ack);

        if request.flags & FLAG_DATA != 0 {
            session.pending_uplink.entry(request.seq).or_insert(request.payload);
            flush_uplink(&mut session).await?;
        }
        if request.flags & FLAG_FIN != 0 {
            session.client_closed = true;
            let _ = session.writer.shutdown().await;
        }

        let mut flags = 0u8;
        let max_down = max_down_payload(&query, request.request_id, session.next_uplink_seq, session.next_down_seq);
        let requested_down = (request.flags & FLAG_DATA == 0).then_some(request.seq);
        let (down_seq, down_payload) = if let Some(chunk) = next_down_chunk(&mut session, max_down, requested_down) {
            flags |= FLAG_DOWNLINK;
            chunk
        } else {
            (0, Vec::new())
        };

        let mut response = crate::protocol::ResponsePacket {
            request_id: request.request_id,
            ack: session.next_uplink_seq,
            flags,
            down_seq,
            auth_tag: [0; crate::auth::AUTH_TAG_LEN],
            payload: down_payload,
        };
        response.sign(&session.client_key)?;
        response
    };
    let wire = build_response(&query, &response)?;
    socket.send_to(&wire, peer).await?;
    Ok(())
}

async fn get_or_create_session(
    shared: Arc<Shared>,
    session_id: u64,
    target: SocketAddr,
    client_key: ClientAccessKey,
) -> Result<Arc<Mutex<Session>>> {
    if let Some(existing) = shared.sessions.lock().await.get(&session_id).cloned() {
        return Ok(existing);
    }

    let stream = TcpStream::connect(target)
        .await
        .with_context(|| format!("connect to target {target}"))?;
    stream.set_nodelay(true)?;
    let (read_half, write_half) = stream.into_split();
    let session = Arc::new(Mutex::new(Session {
        writer: write_half,
        client_key,
        next_uplink_seq: 0,
        pending_uplink: BTreeMap::new(),
        next_down_seq: 0,
        down_queue: VecDeque::new(),
        down_chunks: BTreeMap::new(),
        last_seen: Instant::now(),
        client_closed: false,
        remote_closed: false,
    }));
    shared
        .sessions
        .lock()
        .await
        .insert(session_id, session.clone());
    spawn_down_reader(session.clone(), read_half);
    Ok(session)
}

fn spawn_down_reader(session: Arc<Mutex<Session>>, mut reader: tokio::net::tcp::OwnedReadHalf) {
    let (tx, mut rx) = mpsc::unbounded_channel::<Vec<u8>>();
    tokio::spawn(async move {
        let mut buf = vec![0u8; MAX_RESPONSE_PAYLOAD];
        loop {
            match reader.read(&mut buf).await {
                Ok(0) => {
                    break;
                }
                Ok(len) => {
                    if tx.send(buf[..len].to_vec()).is_err() {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(bytes) = rx.recv().await {
            let mut guard = session.lock().await;
            guard.down_queue.extend(bytes);
        }
        if let Ok(mut guard) = session.try_lock() {
            guard.remote_closed = true;
        }
    });
}

fn apply_down_ack(session: &mut Session, ack: u32) {
    session.down_chunks.retain(|seq, _| *seq >= ack);
}

async fn flush_uplink(session: &mut Session) -> Result<()> {
    while let Some(payload) = session.pending_uplink.remove(&session.next_uplink_seq) {
        session.writer.write_all(&payload).await?;
        session.next_uplink_seq = session.next_uplink_seq.wrapping_add(1);
    }
    Ok(())
}

fn next_down_chunk(session: &mut Session, max_payload: usize, requested_seq: Option<u32>) -> Option<(u32, Vec<u8>)> {
    let requested_seq = requested_seq?;
    materialize_down_chunks(session, requested_seq, max_payload);
    session
        .down_chunks
        .get(&requested_seq)
        .cloned()
        .map(|bytes| (requested_seq, bytes))
}

fn materialize_down_chunks(session: &mut Session, requested_seq: u32, max_payload: usize) {
    let chunk_size = max_payload.min(RESPONSE_CHUNK_SIZE);
    if chunk_size == 0 {
        return;
    }
    while session.next_down_seq <= requested_seq {
        let take = chunk_size.min(session.down_queue.len());
        if take == 0 {
            break;
        }
        let mut bytes = Vec::with_capacity(take);
        for _ in 0..take {
            if let Some(byte) = session.down_queue.pop_front() {
                bytes.push(byte);
            }
        }
        let seq = session.next_down_seq;
        session.next_down_seq = session.next_down_seq.wrapping_add(1);
        session.down_chunks.insert(seq, bytes);
    }
}

fn max_down_payload(query: &crate::protocol::ParsedQuery, request_id: u32, ack: u32, down_seq: u32) -> usize {
    let response = crate::protocol::ResponsePacket {
        request_id,
        ack,
        flags: FLAG_DOWNLINK,
        down_seq,
        auth_tag: [0; crate::auth::AUTH_TAG_LEN],
        payload: Vec::new(),
    };
    let limit = (query.max_payload as usize).min(DNS_MAX_PAYLOAD as usize);
    let base = build_response(query, &response)
        .map(|wire| wire.len())
        .unwrap_or(limit);
    let budget = limit.saturating_sub(base);
    let mut payload = RESPONSE_CHUNK_SIZE.min(budget);
    while payload > 0 && payload + payload.div_ceil(255) > budget {
        payload -= 1;
    }
    payload
}

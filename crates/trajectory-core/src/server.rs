use crate::auth::ClientAccessKey;
use crate::protocol::{
    build_empty_response, build_response, decode_query_request, parse_query, DNS_MAX_PAYLOAD,
    FLAG_DATA, FLAG_DOWNLINK, FLAG_FIN, MAX_RESPONSE_PAYLOAD,
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

const SERVER_DOWNLINK_WINDOW: usize = 96;
const SERVER_RESPONSE_CHUNK_SIZE: usize = 4096;

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub bind: SocketAddr,
    pub domain: String,
    pub target: SocketAddr,
    pub authorized_clients: Arc<HashMap<u32, ClientAccessKey>>,
}

struct Session {
    writer: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>,
    client_key: ClientAccessKey,
    next_uplink_seq: u32,
    pending_uplink: BTreeMap<u32, Vec<u8>>,
    next_down_seq: u32,
    down_queue: VecDeque<u8>,
    down_chunks: BTreeMap<u32, Vec<u8>>,
    down_reserved: BTreeMap<u32, DownReservation>,
    last_seen: Instant,
    last_request_at: Option<Instant>,
    request_gap_ewma: Duration,
    flush_in_progress: bool,
    client_closed: bool,
    writer_shutdown: bool,
    remote_closed: bool,
}

#[derive(Clone, Copy)]
struct DownReservation {
    last_sent_at: Instant,
    send_count: u8,
}

struct Shared {
    sessions: Mutex<HashMap<u64, SessionEntry>>,
}

enum SessionEntry {
    Pending(Arc<tokio::sync::Notify>),
    Ready(Arc<Mutex<Session>>),
}

fn debug_server_log(message: impl AsRef<str>) {
    if std::env::var_os("TRAJECTORY_DEBUG").is_some() {
        eprintln!("[trajectory-server] {}", message.as_ref());
    }
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
            sessions.retain(|_, session| match session {
                SessionEntry::Ready(session) => {
                    if let Ok(guard) = session.try_lock() {
                        guard.last_seen.elapsed() < Duration::from_secs(120)
                    } else {
                        true
                    }
                }
                SessionEntry::Pending(_) => true,
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
    let session =
        get_or_create_session(shared, request.session_id, target, access_key.clone()).await?;
    let response = {
        let mut session = session.lock().await;
        if session.client_key.client_id != access_key.client_id {
            let wire = build_empty_response(&query, ResponseCode::NoError)?;
            socket.send_to(&wire, peer).await?;
            return Ok(());
        }
        let now = Instant::now();
        session.last_seen = now;
        note_request_gap(&mut session, now);
        apply_down_ack(&mut session, request.down_ack);

        if request.flags & FLAG_DATA != 0 && request.seq >= session.next_uplink_seq {
            session
                .pending_uplink
                .entry(request.seq)
                .or_insert(request.payload);
        }
        if request.flags & FLAG_FIN != 0 {
            session.client_closed = true;
        }

        let mut flags = 0u8;
        let max_down = max_down_payload(&query, session.next_uplink_seq, session.next_down_seq);
        let (down_seq, down_payload) = if let Some(chunk) =
            next_scheduled_down_chunk(&mut session, max_down, request.down_ack)
        {
            flags |= FLAG_DOWNLINK;
            chunk
        } else {
            (0, Vec::new())
        };

        let mut response = crate::protocol::ResponsePacket {
            ack: session.next_uplink_seq,
            flags,
            down_seq,
            auth_tag: [0; crate::auth::AUTH_TAG_LEN],
            payload: down_payload,
        };
        response.sign(&session.client_key)?;
        response
    };
    flush_uplink(session.clone()).await?;
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
    loop {
        let pending = {
            let mut sessions = shared.sessions.lock().await;
            match sessions.get(&session_id) {
                Some(SessionEntry::Ready(existing)) => {
                    debug_server_log(format!("reusing session {session_id:016x}"));
                    return Ok(existing.clone());
                }
                Some(SessionEntry::Pending(notify)) => {
                    debug_server_log(format!("waiting for pending session {session_id:016x}"));
                    Some(notify.clone())
                }
                None => {
                    debug_server_log(format!("creating session {session_id:016x}"));
                    let notify = Arc::new(tokio::sync::Notify::new());
                    sessions.insert(session_id, SessionEntry::Pending(notify.clone()));
                    None
                }
            }
        };

        if let Some(notify) = pending {
            notify.notified().await;
            continue;
        }

        let result = async {
            let stream = TcpStream::connect(target)
                .await
                .with_context(|| format!("connect to target {target}"))?;
            stream.set_nodelay(true)?;
            let (read_half, write_half) = stream.into_split();
            let session = Arc::new(Mutex::new(Session {
                writer: Arc::new(Mutex::new(write_half)),
                client_key: client_key.clone(),
                next_uplink_seq: 0,
                pending_uplink: BTreeMap::new(),
                next_down_seq: 0,
                down_queue: VecDeque::new(),
                down_chunks: BTreeMap::new(),
                down_reserved: BTreeMap::new(),
                last_seen: Instant::now(),
                last_request_at: None,
                request_gap_ewma: Duration::from_millis(50),
                flush_in_progress: false,
                client_closed: false,
                writer_shutdown: false,
                remote_closed: false,
            }));
            Ok::<_, anyhow::Error>((session, read_half))
        }
        .await;

        let mut sessions = shared.sessions.lock().await;
        let Some(SessionEntry::Pending(notify)) = sessions.remove(&session_id) else {
            continue;
        };
        match result {
            Ok((session, read_half)) => {
                debug_server_log(format!(
                    "session {session_id:016x} connected to target {target}"
                ));
                sessions.insert(session_id, SessionEntry::Ready(session.clone()));
                notify.notify_waiters();
                drop(sessions);
                spawn_down_reader(session.clone(), read_half);
                return Ok(session);
            }
            Err(error) => {
                debug_server_log(format!(
                    "session {session_id:016x} failed to connect to target {target}: {error:#}"
                ));
                notify.notify_waiters();
                return Err(error);
            }
        }
    }
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

fn note_request_gap(session: &mut Session, now: Instant) {
    if let Some(last_request_at) = session.last_request_at {
        let sample = now.saturating_duration_since(last_request_at);
        let ewma_us =
            (session.request_gap_ewma.as_micros() as u64 * 3 + sample.as_micros() as u64) / 4;
        session.request_gap_ewma = Duration::from_micros(ewma_us.max(1));
    }
    session.last_request_at = Some(now);
}

fn apply_down_ack(session: &mut Session, ack: u32) {
    session.down_chunks.retain(|seq, _| *seq >= ack);
    session.down_reserved.retain(|seq, _| *seq >= ack);
}

async fn flush_uplink(session: Arc<Mutex<Session>>) -> Result<()> {
    loop {
        let (writer, payload, chunk_count, should_shutdown) = {
            let mut session = session.lock().await;
            if session.flush_in_progress {
                return Ok(());
            }
            if let Some((chunk_count, payload)) = take_uplink_batch(&mut session) {
                session.flush_in_progress = true;
                (session.writer.clone(), Some(payload), chunk_count, false)
            } else if session.client_closed && !session.writer_shutdown {
                session.flush_in_progress = true;
                session.writer_shutdown = true;
                (session.writer.clone(), None, 0, true)
            } else {
                return Ok(());
            }
        };

        if should_shutdown {
            let shutdown_result = {
                let mut writer = writer.lock().await;
                writer.shutdown().await
            };
            let mut session = session.lock().await;
            session.flush_in_progress = false;
            shutdown_result?;
            return Ok(());
        }

        let payload = payload.expect("flush payload");
        {
            let mut writer = writer.lock().await;
            writer.write_all(&payload).await?;
        }

        let mut session = session.lock().await;
        session.next_uplink_seq = session.next_uplink_seq.wrapping_add(chunk_count);
        let next_uplink_seq = session.next_uplink_seq;
        session
            .pending_uplink
            .retain(|seq, _| *seq >= next_uplink_seq);
        session.flush_in_progress = false;
    }
}

fn take_uplink_batch(session: &mut Session) -> Option<(u32, Vec<u8>)> {
    let mut seq = session.next_uplink_seq;
    let mut count = 0u32;
    let mut payload = Vec::new();
    while let Some(chunk) = session.pending_uplink.remove(&seq) {
        payload.extend_from_slice(&chunk);
        count = count.wrapping_add(1);
        seq = seq.wrapping_add(1);
    }
    (count > 0).then_some((count, payload))
}

fn next_scheduled_down_chunk(
    session: &mut Session,
    max_payload: usize,
    down_ack: u32,
) -> Option<(u32, Vec<u8>)> {
    let now = Instant::now();
    session.down_reserved.retain(|seq, _| *seq >= down_ack);
    if let Some(chunk) = repair_head_reservation(session, down_ack, now) {
        return Some(chunk);
    }
    let limit = down_ack.saturating_add(SERVER_DOWNLINK_WINDOW as u32);
    let mut seq = down_ack;
    while seq < limit {
        materialize_down_chunks(session, seq, max_payload);
        if !session.down_reserved.contains_key(&seq) {
            if let Some(bytes) = session.down_chunks.get(&seq).cloned() {
                session.down_reserved.insert(
                    seq,
                    DownReservation {
                        last_sent_at: now,
                        send_count: 1,
                    },
                );
                return Some((seq, bytes));
            }
            if seq >= session.next_down_seq {
                break;
            }
        }
        seq = seq.wrapping_add(1);
    }
    None
}

fn repair_head_reservation(
    session: &mut Session,
    down_ack: u32,
    now: Instant,
) -> Option<(u32, Vec<u8>)> {
    let reservation = session.down_reserved.get(&down_ack).copied()?;
    let age = now.duration_since(reservation.last_sent_at);
    let head_repeat_after = clamp_duration(
        session.request_gap_ewma.saturating_mul(4),
        Duration::from_millis(120),
        Duration::from_millis(500),
    );
    let head_release_after = clamp_duration(
        session.request_gap_ewma.saturating_mul(10),
        Duration::from_millis(300),
        Duration::from_millis(1_500),
    );

    // Cumulative ACK means a lost head chunk can deadlock the whole downlink buffer.
    if reservation.send_count == 1 && age >= head_repeat_after {
        if let Some(bytes) = session.down_chunks.get(&down_ack).cloned() {
            session.down_reserved.insert(
                down_ack,
                DownReservation {
                    last_sent_at: now,
                    send_count: 2,
                },
            );
            return Some((down_ack, bytes));
        }
    }

    if age >= head_release_after {
        session.down_reserved.remove(&down_ack);
    }
    None
}

fn clamp_duration(value: Duration, min: Duration, max: Duration) -> Duration {
    value.max(min).min(max)
}

fn materialize_down_chunks(session: &mut Session, requested_seq: u32, max_payload: usize) {
    let chunk_size = max_payload.min(SERVER_RESPONSE_CHUNK_SIZE);
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

fn max_down_payload(query: &crate::protocol::ParsedQuery, ack: u32, down_seq: u32) -> usize {
    let response = crate::protocol::ResponsePacket {
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
    let mut payload = SERVER_RESPONSE_CHUNK_SIZE.min(budget);
    while payload > 0 && payload + payload.div_ceil(255) > budget {
        payload -= 1;
    }
    payload
}

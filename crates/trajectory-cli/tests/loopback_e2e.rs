use std::collections::{HashMap, HashSet};
use std::net::{SocketAddr, TcpListener as StdTcpListener, UdpSocket as StdUdpSocket};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout, Instant};
use trajectory_cli::runtime::{
    run_client, run_server, ClientConfig, ClientMode, ResolverTransportMode, ServerConfig,
    ServerTargetMode,
};
use trajectory_core::auth::ClientAccessKey;
use trajectory_core::codec::{
    open_packet_with_key, seal_packet, AckRange, Direction, Frame, Packet,
};
use trajectory_core::dns::{
    build_query, build_txt_response, envelope_to_qname, parse_query, parse_txt_response,
    qname_to_envelope,
};

fn free_tcp_addr() -> SocketAddr {
    let listener = StdTcpListener::bind("127.0.0.1:0").expect("bind TCP");
    let addr = listener.local_addr().expect("TCP addr");
    drop(listener);
    addr
}

fn free_udp_addr() -> SocketAddr {
    let socket = StdUdpSocket::bind("127.0.0.1:0").expect("bind UDP");
    let addr = socket.local_addr().expect("UDP addr");
    drop(socket);
    addr
}

async fn connect_with_retry(addr: SocketAddr, wait: Duration) -> TcpStream {
    let started = Instant::now();
    loop {
        match TcpStream::connect(addr).await {
            Ok(stream) => return stream,
            Err(error) if started.elapsed() < wait => {
                let _ = error;
                sleep(Duration::from_millis(10)).await;
            }
            Err(error) => panic!("connect client at {addr} before timeout: {error}"),
        }
    }
}

async fn read_http_response_head(stream: &mut TcpStream) -> Vec<u8> {
    let mut data = Vec::new();
    let mut buf = [0u8; 128];
    loop {
        let n = timeout(Duration::from_secs(10), stream.read(&mut buf))
            .await
            .expect("read HTTP response before timeout")
            .expect("read HTTP response");
        assert!(n > 0, "HTTP proxy closed before response headers");
        data.extend_from_slice(&buf[..n]);
        if data.windows(4).any(|window| window == b"\r\n\r\n") {
            return data;
        }
        assert!(data.len() < 4096, "HTTP response header too large");
    }
}

async fn spawn_echo_target(addr: SocketAddr) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await.expect("bind echo");
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                return;
            };
            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                loop {
                    let Ok(n) = stream.read(&mut buf).await else {
                        return;
                    };
                    if n == 0 {
                        return;
                    }
                    if stream.write_all(&buf[..n]).await.is_err() {
                        return;
                    }
                }
            });
        }
    })
}

async fn spawn_socks_forwarder(
    proxy_addr: SocketAddr,
    expected_target: SocketAddr,
    connect_count: Arc<AtomicUsize>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let listener = TcpListener::bind(proxy_addr).await.expect("bind socks");
        loop {
            let Ok((mut inbound, _)) = listener.accept().await else {
                return;
            };
            let connect_count = Arc::clone(&connect_count);
            tokio::spawn(async move {
                connect_count.fetch_add(1, Ordering::Relaxed);
                let mut greeting = [0u8; 3];
                inbound
                    .read_exact(&mut greeting)
                    .await
                    .expect("socks greeting");
                assert_eq!(greeting, [0x05, 0x01, 0x00]);
                inbound
                    .write_all(&[0x05, 0x00])
                    .await
                    .expect("socks method");

                let mut head = [0u8; 4];
                inbound.read_exact(&mut head).await.expect("socks head");
                assert_eq!(&head[..3], &[0x05, 0x01, 0x00]);
                let target = match head[3] {
                    0x01 => {
                        let mut ip = [0u8; 4];
                        inbound.read_exact(&mut ip).await.expect("socks ipv4");
                        let mut port = [0u8; 2];
                        inbound.read_exact(&mut port).await.expect("socks port");
                        SocketAddr::from((ip, u16::from_be_bytes(port)))
                    }
                    0x04 => {
                        let mut ip = [0u8; 16];
                        inbound.read_exact(&mut ip).await.expect("socks ipv6");
                        let mut port = [0u8; 2];
                        inbound.read_exact(&mut port).await.expect("socks port");
                        SocketAddr::from((ip, u16::from_be_bytes(port)))
                    }
                    other => panic!("unexpected socks address type {other}"),
                };
                assert_eq!(target, expected_target);

                let mut outbound = TcpStream::connect(expected_target)
                    .await
                    .expect("connect forwarded target");
                inbound
                    .write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
                    .await
                    .expect("socks connect ok");
                let _ = tokio::io::copy_bidirectional(&mut inbound, &mut outbound).await;
            });
        }
    })
}

async fn spawn_recording_dns(
    addr: SocketAddr,
    domain: String,
    key: ClientAccessKey,
    captured: mpsc::Sender<Packet>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let socket = UdpSocket::bind(addr).await.expect("bind recording DNS");
        let mut buf = vec![0u8; 4096];
        let mut server_packet_no = 0u64;
        loop {
            let Ok((len, peer)) = socket.recv_from(&mut buf).await else {
                return;
            };
            let query = parse_query(&buf[..len]).expect("parse DNS query");
            let envelope = qname_to_envelope(&query.qname, &domain).expect("extract envelope");
            let packet =
                open_packet_with_key(&key, Direction::ClientToServer, &envelope).expect("open");
            let _ = captured.send(packet.clone()).await;

            let mut response = Packet::new(packet.conn_id, server_packet_no);
            server_packet_no = server_packet_no.wrapping_add(1);
            response.max_response_bytes = packet.max_response_bytes;
            response.ack_ranges.push(AckRange {
                first: packet.packet_no,
                last: packet.packet_no,
            });

            let mut stream_offsets = HashMap::<u64, u64>::new();
            for frame in &packet.frames {
                match frame {
                    Frame::Open { stream_id, .. } => {
                        stream_offsets.entry(*stream_id).or_insert(0);
                    }
                    Frame::Data {
                        stream_id,
                        offset,
                        bytes,
                        ..
                    } => {
                        let end = offset.saturating_add(bytes.len() as u64);
                        stream_offsets
                            .entry(*stream_id)
                            .and_modify(|current| *current = (*current).max(end))
                            .or_insert(end);
                    }
                    Frame::StreamAck { stream_id, .. } => {
                        stream_offsets.entry(*stream_id).or_insert(0);
                    }
                    Frame::PathChallenge {
                        nonce,
                        response_bytes,
                    } => response.frames.push(Frame::PathResponse {
                        nonce: *nonce,
                        bytes: vec![0; *response_bytes as usize],
                    }),
                    Frame::Close { .. } | Frame::Ping { .. } | Frame::PathResponse { .. } => {}
                }
            }
            for (stream_id, cumulative_offset) in stream_offsets {
                response.frames.push(Frame::StreamAck {
                    stream_id,
                    cumulative_offset,
                    max_stream_data: 1024 * 1024,
                    fin_offset: None,
                    ranges: Vec::new(),
                });
            }
            if response.frames.is_empty() {
                response.frames.push(Frame::Ping {
                    nonce: packet.packet_no,
                });
            }

            let envelope =
                seal_packet(&key, Direction::ServerToClient, &response).expect("seal response");
            let dns_response =
                build_txt_response(&query, &envelope, 0).expect("build TXT response");
            let _ = socket.send_to(&dns_response, peer).await;
        }
    })
}

async fn send_dns_packet_direct(
    socket: &UdpSocket,
    server: SocketAddr,
    domain: &str,
    key: &ClientAccessKey,
    query_id: u16,
    packet: &Packet,
) -> Packet {
    let envelope = seal_packet(key, Direction::ClientToServer, packet).expect("seal request");
    let qname = envelope_to_qname(&envelope, domain).expect("query name fits");
    let query = build_query(query_id, &qname, 4096).expect("build query");
    socket.send_to(&query, server).await.expect("send query");

    let mut buf = vec![0u8; 8192];
    let (len, _) = timeout(Duration::from_secs(5), socket.recv_from(&mut buf))
        .await
        .expect("receive DNS response before timeout")
        .expect("receive DNS response");
    let envelope = parse_txt_response(&buf[..len]).expect("parse TXT response");
    open_packet_with_key(key, Direction::ServerToClient, &envelope).expect("open response")
}

async fn spawn_classifying_target(
    addr: SocketAddr,
    ready: mpsc::Sender<u8>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await.expect("bind target");
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                return;
            };
            let ready = ready.clone();
            tokio::spawn(async move {
                let mut marker = [0u8; 1];
                stream
                    .read_exact(&mut marker)
                    .await
                    .expect("read stream marker");
                match marker[0] {
                    b'L' => {
                        let payload = vec![b'L'; 4096];
                        stream
                            .write_all(&payload)
                            .await
                            .expect("write large response");
                    }
                    b'S' => stream
                        .write_all(b"small")
                        .await
                        .expect("write small response"),
                    other => panic!("unexpected stream marker {other}"),
                }
                let _ = ready.send(marker[0]).await;
                sleep(Duration::from_secs(60)).await;
            });
        }
    })
}

async fn spawn_stale_tail_target(
    addr: SocketAddr,
    active_response_len: usize,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await.expect("bind stale target");
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                return;
            };
            tokio::spawn(async move {
                let mut marker = [0u8; 1];
                stream
                    .read_exact(&mut marker)
                    .await
                    .expect("read stale target marker");
                match marker[0] {
                    b'Z' => {
                        stream
                            .write_all(b"ok")
                            .await
                            .expect("write stale stream prefix");
                        let tail = vec![b'Z'; 256 * 1024];
                        let _ = stream.write_all(&tail).await;
                        sleep(Duration::from_secs(60)).await;
                    }
                    b'A' => {
                        let payload = vec![b'A'; active_response_len];
                        stream
                            .write_all(&payload)
                            .await
                            .expect("write active response");
                    }
                    other => panic!("unexpected stale target marker {other}"),
                }
            });
        }
    })
}

fn stream_ack(stream_id: u64) -> Frame {
    Frame::StreamAck {
        stream_id,
        cumulative_offset: 0,
        max_stream_data: 1024 * 1024,
        fin_offset: None,
        ranges: Vec::new(),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn raw_tcp_stream_roundtrips_through_dns_udp() {
    let key = ClientAccessKey::generate();
    let target_addr = free_tcp_addr();
    let dns_addr = free_udp_addr();
    let local_addr = free_tcp_addr();
    let domain = "tun.example.test".to_string();

    let echo = spawn_echo_target(target_addr).await;

    let mut registry = HashMap::new();
    registry.insert(key.client_id, key.clone());
    let server = tokio::spawn(run_server(ServerConfig {
        bind: dns_addr,
        domain: domain.clone(),
        target: target_addr,
        target_mode: ServerTargetMode::Tcp,
        authorized_clients: Arc::new(registry),
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let client = tokio::spawn(run_client(ClientConfig {
        listen: local_addr,
        socks_listen: None,
        http_listen: None,
        resolvers: vec![dns_addr],
        domain,
        access_key: key,
        resolver_socks_proxy: None,
        resolver_transport: ResolverTransportMode::Auto,
        poll_interval: Duration::from_millis(5),
        dns_max_payload: 1232,
        admission_report: None,
        resolver_cohort_size: None,
        resolver_admission_min: 1,
        mode: ClientMode::Secure,
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut app = connect_with_retry(local_addr, Duration::from_secs(30)).await;
    let payload = b"trajectory loopback e2e payload";
    app.write_all(payload).await.expect("write app payload");

    let mut got = vec![0u8; payload.len()];
    timeout(Duration::from_secs(10), app.read_exact(&mut got))
        .await
        .expect("read echoed payload before timeout")
        .expect("read echoed payload");
    assert_eq!(got, payload);

    client.abort();
    server.abort();
    echo.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn socks_handshake_survives_dns_chunking() {
    let key = ClientAccessKey::generate();
    let target_addr = free_tcp_addr();
    let dns_addr = free_udp_addr();
    let local_addr = free_tcp_addr();
    let domain = "tun.example.test".to_string();

    let fake_socks = tokio::spawn(async move {
        let listener = TcpListener::bind(target_addr)
            .await
            .expect("bind fake socks");
        let (mut stream, _) = listener.accept().await.expect("accept fake socks");
        let mut greeting = [0u8; 3];
        stream
            .read_exact(&mut greeting)
            .await
            .expect("read greeting");
        assert_eq!(greeting, [0x05, 0x01, 0x00]);
        stream.write_all(&[0x05, 0x00]).await.expect("write method");

        let mut head = [0u8; 5];
        stream
            .read_exact(&mut head)
            .await
            .expect("read connect head");
        assert_eq!(head, [0x05, 0x01, 0x00, 0x03, 0x0b]);
        let mut domain = [0u8; 11];
        stream.read_exact(&mut domain).await.expect("read domain");
        assert_eq!(&domain, b"example.com");
        let mut port = [0u8; 2];
        stream.read_exact(&mut port).await.expect("read port");
        assert_eq!(u16::from_be_bytes(port), 80);
        stream
            .write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
            .await
            .expect("write connect ok");

        let mut payload = [0u8; 4];
        stream.read_exact(&mut payload).await.expect("read payload");
        assert_eq!(&payload, b"ping");
        stream.write_all(b"pong").await.expect("write pong");
    });

    let mut registry = HashMap::new();
    registry.insert(key.client_id, key.clone());
    let server = tokio::spawn(run_server(ServerConfig {
        bind: dns_addr,
        domain: domain.clone(),
        target: target_addr,
        target_mode: ServerTargetMode::Tcp,
        authorized_clients: Arc::new(registry),
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let client = tokio::spawn(run_client(ClientConfig {
        listen: local_addr,
        socks_listen: None,
        http_listen: None,
        resolvers: vec![dns_addr],
        domain,
        access_key: key,
        resolver_socks_proxy: None,
        resolver_transport: ResolverTransportMode::Auto,
        poll_interval: Duration::from_millis(5),
        dns_max_payload: 1232,
        admission_report: None,
        resolver_cohort_size: None,
        resolver_admission_min: 1,
        mode: ClientMode::Secure,
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut app = connect_with_retry(local_addr, Duration::from_secs(30)).await;
    app.write_all(&[0x05, 0x01, 0x00])
        .await
        .expect("write greeting");
    let mut method = [0u8; 2];
    timeout(Duration::from_secs(10), app.read_exact(&mut method))
        .await
        .expect("read method before timeout")
        .expect("read method");
    assert_eq!(method, [0x05, 0x00]);

    let mut connect = vec![0x05, 0x01, 0x00, 0x03, 11];
    connect.extend_from_slice(b"example.com");
    connect.extend_from_slice(&80u16.to_be_bytes());
    app.write_all(&connect).await.expect("write connect");
    let mut reply = [0u8; 10];
    timeout(Duration::from_secs(10), app.read_exact(&mut reply))
        .await
        .expect("read connect reply before timeout")
        .expect("read connect reply");
    assert_eq!(reply[1], 0x00);

    app.write_all(b"ping").await.expect("write payload");
    let mut pong = [0u8; 4];
    timeout(Duration::from_secs(10), app.read_exact(&mut pong))
        .await
        .expect("read pong before timeout")
        .expect("read pong");
    assert_eq!(&pong, b"pong");

    client.abort();
    server.abort();
    fake_socks.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn server_direct_socks5_mode_connects_without_external_proxy() {
    let key = ClientAccessKey::generate();
    let target_addr = free_tcp_addr();
    let dns_addr = free_udp_addr();
    let local_addr = free_tcp_addr();
    let domain = "tun.example.test".to_string();

    let echo = spawn_echo_target(target_addr).await;

    let mut registry = HashMap::new();
    registry.insert(key.client_id, key.clone());
    let server = tokio::spawn(run_server(ServerConfig {
        bind: dns_addr,
        domain: domain.clone(),
        target: target_addr,
        target_mode: ServerTargetMode::Socks5Direct,
        authorized_clients: Arc::new(registry),
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let client = tokio::spawn(run_client(ClientConfig {
        listen: local_addr,
        socks_listen: None,
        http_listen: None,
        resolvers: vec![dns_addr],
        domain,
        access_key: key,
        resolver_socks_proxy: None,
        resolver_transport: ResolverTransportMode::Auto,
        poll_interval: Duration::from_millis(5),
        dns_max_payload: 1232,
        admission_report: None,
        resolver_cohort_size: None,
        resolver_admission_min: 1,
        mode: ClientMode::Secure,
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut app = connect_with_retry(local_addr, Duration::from_secs(30)).await;
    app.write_all(&[0x05, 0x01, 0x00])
        .await
        .expect("write greeting");
    let mut method = [0u8; 2];
    timeout(Duration::from_secs(10), app.read_exact(&mut method))
        .await
        .expect("read method before timeout")
        .expect("read method");
    assert_eq!(method, [0x05, 0x00]);

    let mut connect = vec![0x05, 0x01, 0x00, 0x01];
    connect.extend_from_slice(&[127, 0, 0, 1]);
    connect.extend_from_slice(&target_addr.port().to_be_bytes());
    app.write_all(&connect).await.expect("write connect");
    let mut reply = [0u8; 10];
    timeout(Duration::from_secs(10), app.read_exact(&mut reply))
        .await
        .expect("read connect reply before timeout")
        .expect("read connect reply");
    assert_eq!(reply[1], 0x00);

    app.write_all(b"ping").await.expect("write payload");
    let mut pong = [0u8; 4];
    timeout(Duration::from_secs(10), app.read_exact(&mut pong))
        .await
        .expect("read pong before timeout")
        .expect("read pong");
    assert_eq!(&pong, b"ping");

    client.abort();
    server.abort();
    echo.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn socks_proxy_mode_uses_direct_tunnel_open() {
    let key = ClientAccessKey::generate();
    let target_addr = free_tcp_addr();
    let dns_addr = free_udp_addr();
    let raw_local_addr = free_tcp_addr();
    let socks_local_addr = free_tcp_addr();
    let domain = "tun.example.test".to_string();

    let echo = spawn_echo_target(target_addr).await;

    let mut registry = HashMap::new();
    registry.insert(key.client_id, key.clone());
    let server = tokio::spawn(run_server(ServerConfig {
        bind: dns_addr,
        domain: domain.clone(),
        target: target_addr,
        target_mode: ServerTargetMode::Socks5Direct,
        authorized_clients: Arc::new(registry),
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let client = tokio::spawn(run_client(ClientConfig {
        listen: raw_local_addr,
        socks_listen: Some(socks_local_addr),
        http_listen: None,
        resolvers: vec![dns_addr],
        domain,
        access_key: key,
        resolver_socks_proxy: None,
        resolver_transport: ResolverTransportMode::Auto,
        poll_interval: Duration::from_millis(5),
        dns_max_payload: 1232,
        admission_report: None,
        resolver_cohort_size: None,
        resolver_admission_min: 1,
        mode: ClientMode::Secure,
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut app = connect_with_retry(socks_local_addr, Duration::from_secs(30)).await;
    app.write_all(&[0x05, 0x01, 0x00])
        .await
        .expect("write greeting");
    let mut method = [0u8; 2];
    timeout(Duration::from_secs(10), app.read_exact(&mut method))
        .await
        .expect("read method before timeout")
        .expect("read method");
    assert_eq!(method, [0x05, 0x00]);

    let mut connect = vec![0x05, 0x01, 0x00, 0x01];
    connect.extend_from_slice(&[127, 0, 0, 1]);
    connect.extend_from_slice(&target_addr.port().to_be_bytes());
    app.write_all(&connect).await.expect("write connect");
    let mut reply = [0u8; 10];
    timeout(Duration::from_secs(10), app.read_exact(&mut reply))
        .await
        .expect("read connect reply before timeout")
        .expect("read connect reply");
    assert_eq!(reply[1], 0x00);

    app.write_all(b"ping").await.expect("write payload");
    let mut pong = [0u8; 4];
    timeout(Duration::from_secs(10), app.read_exact(&mut pong))
        .await
        .expect("read pong before timeout")
        .expect("read pong");
    assert_eq!(&pong, b"ping");

    client.abort();
    server.abort();
    echo.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_connect_proxy_mode_uses_direct_tunnel_open() {
    let key = ClientAccessKey::generate();
    let target_addr = free_tcp_addr();
    let dns_addr = free_udp_addr();
    let raw_local_addr = free_tcp_addr();
    let http_local_addr = free_tcp_addr();
    let domain = "tun.example.test".to_string();

    let echo = spawn_echo_target(target_addr).await;

    let mut registry = HashMap::new();
    registry.insert(key.client_id, key.clone());
    let server = tokio::spawn(run_server(ServerConfig {
        bind: dns_addr,
        domain: domain.clone(),
        target: target_addr,
        target_mode: ServerTargetMode::Socks5Direct,
        authorized_clients: Arc::new(registry),
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let client = tokio::spawn(run_client(ClientConfig {
        listen: raw_local_addr,
        socks_listen: None,
        http_listen: Some(http_local_addr),
        resolvers: vec![dns_addr],
        domain,
        access_key: key,
        resolver_socks_proxy: None,
        resolver_transport: ResolverTransportMode::Auto,
        poll_interval: Duration::from_millis(5),
        dns_max_payload: 1232,
        admission_report: None,
        resolver_cohort_size: None,
        resolver_admission_min: 1,
        mode: ClientMode::Secure,
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut app = connect_with_retry(http_local_addr, Duration::from_secs(30)).await;
    app.write_all(
        format!(
            "CONNECT {target_addr} HTTP/1.1\r\nHost: {target_addr}\r\nProxy-Connection: keep-alive\r\n\r\n"
        )
        .as_bytes(),
    )
    .await
    .expect("write CONNECT request");
    let response = read_http_response_head(&mut app).await;
    assert!(
        response.starts_with(b"HTTP/1.1 200 "),
        "unexpected HTTP proxy response: {}",
        String::from_utf8_lossy(&response)
    );

    app.write_all(b"ping").await.expect("write tunnel payload");
    let mut pong = [0u8; 4];
    timeout(Duration::from_secs(10), app.read_exact(&mut pong))
        .await
        .expect("read echoed tunnel payload before timeout")
        .expect("read echoed tunnel payload");
    assert_eq!(&pong, b"ping");

    client.abort();
    server.abort();
    echo.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn dns_tcp_over_socks_reuses_persistent_connection() {
    let key = ClientAccessKey::generate();
    let target_addr = free_tcp_addr();
    let dns_addr = free_tcp_addr();
    let proxy_addr = free_tcp_addr();
    let local_addr = free_tcp_addr();
    let domain = "tun.example.test".to_string();

    let echo = spawn_echo_target(target_addr).await;
    let socks_connects = Arc::new(AtomicUsize::new(0));
    let socks = spawn_socks_forwarder(proxy_addr, dns_addr, Arc::clone(&socks_connects)).await;

    let mut registry = HashMap::new();
    registry.insert(key.client_id, key.clone());
    let server = tokio::spawn(run_server(ServerConfig {
        bind: dns_addr,
        domain: domain.clone(),
        target: target_addr,
        target_mode: ServerTargetMode::Tcp,
        authorized_clients: Arc::new(registry),
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let client = tokio::spawn(run_client(ClientConfig {
        listen: local_addr,
        socks_listen: None,
        http_listen: None,
        resolvers: vec![dns_addr],
        domain,
        access_key: key,
        resolver_socks_proxy: Some(proxy_addr),
        resolver_transport: ResolverTransportMode::Tcp,
        poll_interval: Duration::from_millis(5),
        dns_max_payload: 1232,
        admission_report: None,
        resolver_cohort_size: None,
        resolver_admission_min: 1,
        mode: ClientMode::Secure,
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut app = connect_with_retry(local_addr, Duration::from_secs(30)).await;
    let payload = vec![b'x'; 768];
    app.write_all(&payload).await.expect("write app payload");

    let mut got = vec![0u8; payload.len()];
    timeout(Duration::from_secs(20), app.read_exact(&mut got))
        .await
        .expect("read echoed payload before timeout")
        .expect("read echoed payload");
    assert_eq!(got, payload);
    let connects_after_first_stream = socks_connects.load(Ordering::Relaxed);
    assert!(
        (2..=17).contains(&connects_after_first_stream),
        "proxy DNS path should open a bounded lane set, got {connects_after_first_stream}"
    );

    let second_payload = vec![b'y'; 768];
    app.write_all(&second_payload)
        .await
        .expect("write second app payload");
    let mut second_got = vec![0u8; second_payload.len()];
    timeout(Duration::from_secs(20), app.read_exact(&mut second_got))
        .await
        .expect("read second echoed payload before timeout")
        .expect("read second echoed payload");
    assert_eq!(second_got, second_payload);
    assert_eq!(
        socks_connects.load(Ordering::Relaxed),
        connects_after_first_stream
    );

    client.abort();
    server.abort();
    socks.abort();
    echo.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn regression_closed_streams_do_not_starve_followup_stream() {
    let key = ClientAccessKey::generate();
    let target_addr = free_tcp_addr();
    let dns_addr = free_udp_addr();
    let local_addr = free_tcp_addr();
    let domain = "tun.example.test".to_string();
    let active_response_len = 24 * 1024;

    let target = spawn_stale_tail_target(target_addr, active_response_len).await;

    let mut registry = HashMap::new();
    registry.insert(key.client_id, key.clone());
    let server = tokio::spawn(run_server(ServerConfig {
        bind: dns_addr,
        domain: domain.clone(),
        target: target_addr,
        target_mode: ServerTargetMode::Tcp,
        authorized_clients: Arc::new(registry),
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let client = tokio::spawn(run_client(ClientConfig {
        listen: local_addr,
        socks_listen: None,
        http_listen: None,
        resolvers: vec![dns_addr],
        domain,
        access_key: key,
        resolver_socks_proxy: None,
        resolver_transport: ResolverTransportMode::Auto,
        poll_interval: Duration::from_millis(5),
        dns_max_payload: 512,
        admission_report: None,
        resolver_cohort_size: None,
        resolver_admission_min: 1,
        mode: ClientMode::Secure,
    }));

    tokio::time::sleep(Duration::from_millis(50)).await;

    for _ in 0..2 {
        let mut stale = connect_with_retry(local_addr, Duration::from_secs(30)).await;
        stale
            .write_all(b"Z")
            .await
            .expect("write stale stream marker");
        let mut ok = [0u8; 2];
        timeout(Duration::from_secs(10), stale.read_exact(&mut ok))
            .await
            .expect("read stale stream prefix before timeout")
            .expect("read stale stream prefix");
        assert_eq!(&ok, b"ok");
        drop(stale);
    }

    let mut active = connect_with_retry(local_addr, Duration::from_secs(30)).await;
    active
        .write_all(b"A")
        .await
        .expect("write active stream marker");
    let mut got = vec![0u8; active_response_len];
    timeout(Duration::from_secs(10), active.read_exact(&mut got))
        .await
        .expect("read active response before stale streams can starve it")
        .expect("read active response");
    assert_eq!(got, vec![b'A'; active_response_len]);

    client.abort();
    server.abort();
    target.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn acceptance_concurrent_streams_share_one_transport_conn_id() {
    let key = ClientAccessKey::generate();
    let dns_addr = free_udp_addr();
    let local_addr = free_tcp_addr();
    let domain = "tun.example.test".to_string();
    let (captured_tx, mut captured_rx) = mpsc::channel(128);

    let dns = spawn_recording_dns(dns_addr, domain.clone(), key.clone(), captured_tx).await;
    sleep(Duration::from_millis(50)).await;

    let client = tokio::spawn(run_client(ClientConfig {
        listen: local_addr,
        socks_listen: None,
        http_listen: None,
        resolvers: vec![dns_addr],
        domain,
        access_key: key,
        resolver_socks_proxy: None,
        resolver_transport: ResolverTransportMode::Auto,
        poll_interval: Duration::from_millis(5),
        dns_max_payload: 1232,
        admission_report: None,
        resolver_cohort_size: None,
        resolver_admission_min: 1,
        mode: ClientMode::Secure,
    }));
    sleep(Duration::from_millis(50)).await;

    let mut first = connect_with_retry(local_addr, Duration::from_secs(30)).await;
    let mut second = connect_with_retry(local_addr, Duration::from_secs(30)).await;
    first.write_all(b"a").await.expect("write first stream");
    second.write_all(b"b").await.expect("write second stream");

    let mut opens = Vec::<(u64, u64)>::new();
    timeout(Duration::from_secs(5), async {
        while opens.len() < 2 {
            let packet = captured_rx.recv().await.expect("capture packet");
            for frame in packet.frames {
                if let Frame::Open { stream_id, .. } = frame {
                    opens.push((packet.conn_id, stream_id));
                }
            }
        }
    })
    .await
    .expect("capture two stream opens");

    let stream_ids = opens
        .iter()
        .map(|(_, stream_id)| *stream_id)
        .collect::<HashSet<_>>();
    let conn_ids = opens
        .iter()
        .map(|(conn_id, _)| *conn_id)
        .collect::<HashSet<_>>();
    assert_eq!(stream_ids.len(), 2, "expected two distinct logical streams");
    assert_eq!(
        conn_ids.len(),
        1,
        "concurrent local streams must share one transport conn_id"
    );

    client.abort();
    dns.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn acceptance_server_packs_control_for_multiple_streams_in_one_response() {
    let key = ClientAccessKey::generate();
    let target_addr = free_tcp_addr();
    let dns_addr = free_udp_addr();
    let domain = "tun.example.test".to_string();

    let echo = spawn_echo_target(target_addr).await;
    let mut registry = HashMap::new();
    registry.insert(key.client_id, key.clone());
    let server = tokio::spawn(run_server(ServerConfig {
        bind: dns_addr,
        domain: domain.clone(),
        target: target_addr,
        target_mode: ServerTargetMode::Tcp,
        authorized_clients: Arc::new(registry),
    }));
    sleep(Duration::from_millis(50)).await;

    let socket = UdpSocket::bind("127.0.0.1:0")
        .await
        .expect("bind client UDP");
    let mut request = Packet::new(0xabc, 0);
    request.max_response_bytes = 1232;
    request.frames.push(Frame::Open {
        stream_id: 1,
        host: String::new(),
        port: 0,
    });
    request.frames.push(Frame::Open {
        stream_id: 2,
        host: String::new(),
        port: 0,
    });

    let response = send_dns_packet_direct(&socket, dns_addr, &domain, &key, 100, &request).await;
    let acked_streams = response
        .frames
        .iter()
        .filter_map(|frame| match frame {
            Frame::StreamAck { stream_id, .. } => Some(*stream_id),
            _ => None,
        })
        .collect::<HashSet<_>>();

    assert!(
        acked_streams.contains(&1) && acked_streams.contains(&2),
        "one response should pack control for every active stream; got {acked_streams:?}"
    );

    server.abort();
    echo.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn acceptance_small_stream_is_not_starved_by_large_stream_backlog() {
    let key = ClientAccessKey::generate();
    let target_addr = free_tcp_addr();
    let dns_addr = free_udp_addr();
    let domain = "tun.example.test".to_string();
    let (ready_tx, mut ready_rx) = mpsc::channel(2);

    let target = spawn_classifying_target(target_addr, ready_tx).await;
    let mut registry = HashMap::new();
    registry.insert(key.client_id, key.clone());
    let server = tokio::spawn(run_server(ServerConfig {
        bind: dns_addr,
        domain: domain.clone(),
        target: target_addr,
        target_mode: ServerTargetMode::Tcp,
        authorized_clients: Arc::new(registry),
    }));
    sleep(Duration::from_millis(50)).await;

    let socket = UdpSocket::bind("127.0.0.1:0")
        .await
        .expect("bind client UDP");
    let conn_id = 0xdef;
    let mut open_large = Packet::new(conn_id, 0);
    open_large.max_response_bytes = 700;
    open_large.frames.push(Frame::Open {
        stream_id: 1,
        host: String::new(),
        port: 0,
    });
    open_large.frames.push(Frame::Data {
        stream_id: 1,
        offset: 0,
        fin: false,
        bytes: b"L".to_vec(),
    });
    let _ = send_dns_packet_direct(&socket, dns_addr, &domain, &key, 200, &open_large).await;

    let mut open_small = Packet::new(conn_id, 1);
    open_small.max_response_bytes = 700;
    open_small.frames.push(Frame::Open {
        stream_id: 2,
        host: String::new(),
        port: 0,
    });
    open_small.frames.push(Frame::Data {
        stream_id: 2,
        offset: 0,
        fin: false,
        bytes: b"S".to_vec(),
    });
    let _ = send_dns_packet_direct(&socket, dns_addr, &domain, &key, 201, &open_small).await;

    let mut ready = HashSet::new();
    timeout(Duration::from_secs(5), async {
        while ready.len() < 2 {
            ready.insert(ready_rx.recv().await.expect("target response ready"));
        }
    })
    .await
    .expect("target prepared both stream responses");

    let mut poll = Packet::new(conn_id, 2);
    poll.max_response_bytes = 700;
    poll.stream_ack_offset = Some(0);
    poll.frames.push(stream_ack(2));
    poll.frames.push(stream_ack(1));
    let response = send_dns_packet_direct(&socket, dns_addr, &domain, &key, 202, &poll).await;

    let small_stream_bytes = response.frames.iter().any(|frame| {
        matches!(
            frame,
            Frame::Data {
                stream_id: 2,
                bytes,
                ..
            } if bytes.windows(b"small".len()).any(|window| window == b"small")
        )
    });
    assert!(
        small_stream_bytes,
        "small stream data should be scheduled even while stream 1 has a large backlog; response frames were {:?}",
        response.frames
    );

    server.abort();
    target.abort();
}

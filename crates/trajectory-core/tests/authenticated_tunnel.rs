use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener as StdTcpListener, UdpSocket as StdUdpSocket};
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::{sleep, timeout};
use trajectory_core::auth::ClientAccessKey;
use trajectory_core::client::{default_client_config, run_until as run_client_until};
use trajectory_core::server::{run_until as run_server_until, ServerConfig};

fn free_tcp_addr() -> SocketAddr {
    let listener = StdTcpListener::bind("127.0.0.1:0").expect("bind ephemeral tcp");
    let addr = listener.local_addr().expect("read tcp addr");
    drop(listener);
    addr
}

fn free_udp_addr() -> SocketAddr {
    let socket = StdUdpSocket::bind("127.0.0.1:0").expect("bind ephemeral udp");
    let addr = socket.local_addr().expect("read udp addr");
    drop(socket);
    addr
}

async fn spawn_echo_target(bind: SocketAddr) -> JoinHandle<()> {
    tokio::spawn(async move {
        let listener = TcpListener::bind(bind).await.expect("bind echo target");
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                let Ok(len) = stream.read(&mut buf).await else {
                    return;
                };
                if len == 0 {
                    return;
                }
                let _ = stream.write_all(b"pong:").await;
                let _ = stream.write_all(&buf[..len]).await;
                let _ = stream.shutdown().await;
            });
        }
    })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn authenticated_client_and_server_forward_tcp() {
    let access_key = ClientAccessKey::generate();
    let echo_addr = free_tcp_addr();
    let server_addr = free_udp_addr();
    let client_addr = free_tcp_addr();

    let echo_task = spawn_echo_target(echo_addr).await;

    let authorized_clients = Arc::new(HashMap::from([(access_key.client_id, access_key.clone())]));
    let server_config = ServerConfig {
        bind: server_addr,
        domain: "t.test".to_owned(),
        target: echo_addr,
        authorized_clients,
    };
    let (server_shutdown_tx, server_shutdown_rx) = watch::channel(false);
    let server_task = tokio::spawn(async move {
        run_server_until(server_config, server_shutdown_rx)
            .await
            .expect("server should run");
    });

    let mut client_config =
        default_client_config(client_addr, vec![server_addr], "t.test".to_owned(), access_key);
    client_config.request_timeout = Duration::from_millis(250);
    let (client_shutdown_tx, client_shutdown_rx) = watch::channel(false);
    let client_task = tokio::spawn(async move {
        run_client_until(client_config, client_shutdown_rx)
            .await
            .expect("client should run");
    });

    sleep(Duration::from_millis(500)).await;

    let mut stream = timeout(Duration::from_secs(5), TcpStream::connect(client_addr))
        .await
        .expect("tcp connect timeout")
        .expect("tcp connect failed");
    timeout(Duration::from_secs(5), stream.write_all(b"ping"))
        .await
        .expect("tcp write timeout")
        .expect("tcp write failed");
    let mut response = vec![0u8; 16];
    let len = timeout(Duration::from_secs(5), stream.read(&mut response))
        .await
        .expect("tcp read timeout")
        .expect("tcp read failed");
    assert_eq!(&response[..len], b"pong:ping");

    let _ = client_shutdown_tx.send(true);
    let _ = server_shutdown_tx.send(true);
    timeout(Duration::from_secs(5), client_task)
        .await
        .expect("client shutdown timeout")
        .expect("client shutdown failed");
    timeout(Duration::from_secs(5), server_task)
        .await
        .expect("server shutdown timeout")
        .expect("server shutdown failed");
    echo_task.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn unauthorized_client_is_rejected_by_the_server() {
    let authorized_key = ClientAccessKey::generate();
    let unauthorized_key = ClientAccessKey::generate();
    let echo_addr = free_tcp_addr();
    let server_addr = free_udp_addr();
    let client_addr = free_tcp_addr();

    let echo_task = spawn_echo_target(echo_addr).await;

    let authorized_clients = Arc::new(HashMap::from([(authorized_key.client_id, authorized_key)]));
    let server_config = ServerConfig {
        bind: server_addr,
        domain: "t.test".to_owned(),
        target: echo_addr,
        authorized_clients,
    };
    let (server_shutdown_tx, server_shutdown_rx) = watch::channel(false);
    let server_task = tokio::spawn(async move {
        run_server_until(server_config, server_shutdown_rx)
            .await
            .expect("server should run");
    });

    let mut client_config = default_client_config(
        client_addr,
        vec![server_addr],
        "t.test".to_owned(),
        unauthorized_key,
    );
    client_config.request_timeout = Duration::from_millis(200);
    let (client_shutdown_tx, client_shutdown_rx) = watch::channel(false);
    let client_task = tokio::spawn(async move {
        run_client_until(client_config, client_shutdown_rx)
            .await
            .expect("client should run");
    });

    sleep(Duration::from_millis(500)).await;

    let mut stream = timeout(Duration::from_secs(5), TcpStream::connect(client_addr))
        .await
        .expect("tcp connect timeout")
        .expect("tcp connect failed");
    timeout(Duration::from_secs(5), stream.write_all(b"ping"))
        .await
        .expect("tcp write timeout")
        .expect("tcp write failed");

    let mut response = vec![0u8; 16];
    let outcome = timeout(Duration::from_secs(2), stream.read(&mut response)).await;
    assert!(outcome.is_err(), "unauthorized client unexpectedly forwarded traffic");

    let _ = client_shutdown_tx.send(true);
    let _ = server_shutdown_tx.send(true);
    timeout(Duration::from_secs(5), client_task)
        .await
        .expect("client shutdown timeout")
        .expect("client shutdown failed");
    timeout(Duration::from_secs(5), server_task)
        .await
        .expect("server shutdown timeout")
        .expect("server shutdown failed");
    echo_task.abort();
}

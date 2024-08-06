use log::info;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::UdpSocket;

static LOG_TARGET: &str = "network_discovery";

pub async fn start_network_discovery_receiver() {
    info!(target: LOG_TARGET, "Binding UDP listener to 0.0.0.0:5005");
    let local_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 5005));

    info!(target: LOG_TARGET, "Initializing socket");
    let socket = UdpSocket::bind(local_addr).await.unwrap();

    info!(target: LOG_TARGET, "Enabling broadcasting on socket");
    socket.set_broadcast(true).unwrap();

    let mut buf = [0; 1024];

    loop {
        info!(target: LOG_TARGET, "Waiting for UDP packets");
        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();

        info!(target: LOG_TARGET, "Parsing received UDP packet from {addr}");
        let msg = String::from_utf8_lossy(&buf[..len]);

        info!(target: LOG_TARGET, "Got UDP packet from {addr}: {msg}");

        info!(target: LOG_TARGET, "Replying to {addr} with current metadata");
        socket.send_to("master".as_bytes(), addr).await.unwrap();
    }
}

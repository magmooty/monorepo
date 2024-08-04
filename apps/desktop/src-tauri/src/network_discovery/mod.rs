use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::UdpSocket;

pub async fn start_network_discovery_receiver() {
    // Bind the UDP socket to the broadcast address and port
    let local_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 5005));
    let socket = UdpSocket::bind(local_addr).await.unwrap();

    // Enable broadcast
    socket.set_broadcast(true).unwrap();

    let mut buf = [0; 1024];

    loop {
        // Receive a message
        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();

        // Convert the message to a string and print it
        let msg = String::from_utf8_lossy(&buf[..len]);
        println!("Received from {}: {}", addr, msg);

        socket.send_to("master".as_bytes(), addr).await.unwrap();
    }
}

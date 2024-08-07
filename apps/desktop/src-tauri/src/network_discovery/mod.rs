use futures::future::join_all;
use log::{debug, info, warn};
use pnet::datalink;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::IpAddr;
use std::time::Duration;
use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};
use tokio::net::UdpSocket;
use tokio::time::timeout;

use crate::app::{get_global_key, GlobalKey};

static LOG_TARGET: &str = "network_discovery";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstanceType {
    Master,
    Slave,
    Uninitialized,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInstanceInfo {
    pub center_name: String,
    pub version: String,
    pub instance_type: InstanceType,
    pub local_center_initialized: bool,
    pub ip_addresses: Vec<Ipv4Addr>,
}

async fn get_current_instance_info() -> NetworkInstanceInfo {
    let center_name = get_global_key(GlobalKey::CenterName).await;

    let version = env!("CARGO_PKG_VERSION").to_string();

    let instance_type = get_global_key(GlobalKey::InstanceType).await;
    let instance_type = instance_type
        .map(|instance_type| {
            serde_json::from_str(&instance_type).unwrap_or(InstanceType::Uninitialized)
        })
        .unwrap_or(InstanceType::Uninitialized);

    let local_center_initialized = center_name.is_some();

    let ip_addresses = get_current_ip_addresses()
        .await
        .iter()
        .map(|(ip, _)| *ip)
        .collect();

    NetworkInstanceInfo {
        center_name: center_name.unwrap_or("".to_string()),
        version,
        instance_type: instance_type,
        local_center_initialized,
        ip_addresses,
    }
}

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

        debug!(target: LOG_TARGET, "Fetching current instance metadata");
        let current_instance_info = get_current_instance_info().await;

        debug!(target: LOG_TARGET, "Serializing current instance metadata into JSON");
        let current_instance_info = serde_json::to_string(&current_instance_info).unwrap();

        info!(target: LOG_TARGET, "Replying to {addr} with current metadata");
        socket
            .send_to(current_instance_info.as_bytes(), addr)
            .await
            .unwrap();
    }
}

async fn get_current_ip_addresses() -> Vec<(Ipv4Addr, Ipv4Addr)> {
    tokio::task::spawn_blocking(|| {
        let mut ip_addresses: Vec<(Ipv4Addr, Ipv4Addr)> = Vec::new();
        let interfaces = datalink::interfaces();

        for interface in interfaces {
            for ip_network in interface.ips {
                if let IpAddr::V4(ipv4) = ip_network.ip() {
                    if !ipv4.is_loopback() {
                        if let IpAddr::V4(mask) = ip_network.mask() {
                            info!(
                                "Found network interface: {}, IP: {}, Mask: {}",
                                interface.name, ipv4, mask
                            );

                            ip_addresses.push((ipv4, mask));
                        }
                    }
                }
            }
        }

        ip_addresses
    })
    .await
    .unwrap()
}

fn calculate_broadcast_address(ip: Ipv4Addr, mask: Ipv4Addr) -> Ipv4Addr {
    let ip_u32 = u32::from(ip);
    let mask_u32 = u32::from(mask);
    let broadcast_u32 = ip_u32 | !mask_u32;
    Ipv4Addr::from(broadcast_u32)
}

fn filter_network_instances(
    instances: Vec<NetworkInstanceInfo>,
    current_ip_addresses: Vec<Ipv4Addr>,
) -> Vec<NetworkInstanceInfo> {
    let current_ip_addresses: HashSet<Ipv4Addr> = current_ip_addresses.into_iter().collect();

    instances
        .into_iter()
        .filter(|instance| {
            instance
                .ip_addresses
                .iter()
                .all(|instance_ip_address| !current_ip_addresses.contains(instance_ip_address))
        })
        .collect()
}

pub async fn discover_network() -> Result<Vec<NetworkInstanceInfo>, io::Error> {
    info!(target: LOG_TARGET, "Starting network discovery");

    debug!(target: LOG_TARGET, "Initializing UDP socket");
    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    // Enable broadcast
    socket.set_broadcast(true)?;

    let ip_addresses = get_current_ip_addresses().await;

    let broadcasted_packets = ip_addresses.iter().map(|(ip, mask)| async {
        let broadcast_addr = calculate_broadcast_address(ip.clone(), mask.clone());
        let message = b"Hello, world!";

        info!(
            target: LOG_TARGET,
            "Broadcasting to {broadcast_addr}"
        );

        socket
            .send_to(message, SocketAddr::new(IpAddr::V4(broadcast_addr), 5005))
            .await
            .unwrap_or_default();

        info!(target: LOG_TARGET, "Waiting for reply");

        let mut buf: [u8; 1024] = [0; 1024];

        match timeout(Duration::new(1, 0), socket.recv_from(&mut buf)).await {
            Ok(Ok((len, addr))) => {
                info!(target: LOG_TARGET, "Parsing reply from {addr}");
                let msg = String::from_utf8_lossy(&buf[..len]).to_string();

                debug!(target: LOG_TARGET, "Received reply from {addr}: {msg}");

                let center_info: Result<NetworkInstanceInfo, serde_json::Error> =
                    serde_json::from_str(msg.as_str());

                match center_info {
                    Ok(center_info) => {
                        info!(target: LOG_TARGET, "Found a network instance: {}", &center_info.center_name);
                        Some(center_info)
                    }
                    Err(error) => {
                        warn!(target: LOG_TARGET, "Failed to parse reply from {addr}: {error}");
                        None
                    }
                }
            }
            Ok(Err(error)) => {
                warn!(target: LOG_TARGET, "Failed to receive reply: {error}");
                None
            }
            Err(_) => {
                info!(target: LOG_TARGET, "Timed out waiting for reply, sending again");
                None
            }
        }
    });

    let instances: Vec<NetworkInstanceInfo> = join_all(broadcasted_packets)
        .await
        .into_iter()
        .filter_map(|instance| instance)
        .collect();

    let ip_addresses = ip_addresses.into_iter().map(|(ip, _)| ip).collect();

    let instances = filter_network_instances(instances, ip_addresses);

    Ok(instances)
}

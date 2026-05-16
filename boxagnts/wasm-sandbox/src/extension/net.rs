use std::net::SocketAddr;
use std::sync::Arc;


use futures_util::FutureExt as _;
use ip_network::IpNetwork;

use wasmtime_wasi::sockets::SocketAddrUse;

use super::outbound_networking_config::allowed_hosts::AllowedHostsConfig;
use super::outbound_networking_config::allowed_hosts::DisallowedHostHandler;
use super::outbound_networking_config::allowed_hosts::OutboundAllowedHosts;
use super::outbound_networking_config::blocked_networks::BlockedNetworks;

pub fn parse_allowed_outbound_hosts(hosts: Vec<String>) -> OutboundAllowedHosts {
    let allowed_hosts_future = async move {
        let resolver = super::expressions::PreparedResolver::default();

        AllowedHostsConfig::parse(&hosts, &resolver).inspect_err(|_err| {
            eprintln!("Error parsing allowed outbound hosts");
        })
    }
    .map(|res| res.map(Arc::new).map_err(Arc::new))
    .boxed()
    .shared();

    let disallowed_host_handler = move |scheme: &str, authority: &str| {
        let host_pattern = format!("{scheme}://{authority}");
        if scheme.starts_with("http") && authority == "self" {
            eprintln!(
                "A component tried to make an HTTP request to its own app but it does not have permission."
            );
        } else {
            eprintln!(
                "A component tried to make an outbound network connection to disallowed destination '{host_pattern}'."
            );
        };
        eprintln!(
            "To allow this request, add '{host_pattern}' to the allowed outbound hosts config."
        );
    };

    let disallowed_host_handler: Option<Arc<dyn DisallowedHostHandler>> =
        Some(Arc::new(disallowed_host_handler));

    OutboundAllowedHosts::new(
        allowed_hosts_future.clone(),
        disallowed_host_handler.clone(),
    )
}

pub fn parse_block_networks(networks: Vec<String>) -> BlockedNetworks {
    let mut block_networks: Vec<IpNetwork> = Vec::new();
    let mut block_private_networks = false;

    for network in networks {
        if network.as_str() == "private" {
            block_private_networks = true;
        }
        if let Ok(ip_network) = IpNetwork::from_str_truncate(network.as_str()) {
            block_networks.push(ip_network);
        }
    }

    BlockedNetworks::new(block_networks, block_private_networks)
}

pub async fn socket_addr_check(
    addr: SocketAddr,
    addr_use: SocketAddrUse,
    allowed_outbound_hosts: OutboundAllowedHosts,
    blocked_networks: BlockedNetworks,
) -> bool {
    let scheme = match addr_use {
        SocketAddrUse::TcpBind => {
            eprintln!("Deny TCP bind: {}", addr);

            return false;
        }
        SocketAddrUse::UdpBind => {
            eprintln!("Deny UDP bind: {}", addr);

            return false;
        }
        SocketAddrUse::TcpConnect => "tcp",
        SocketAddrUse::UdpConnect => "udp",
        SocketAddrUse::UdpOutgoingDatagram => "udp",
    };
    if !allowed_outbound_hosts
        .check_url(&addr.to_string(), scheme)
        .await
        .unwrap_or(false)
    {
        let host_pattern = format!("{scheme}://{addr}");

        eprintln!(
            "A component tried to make an outbound network connection to disallowed destination '{host_pattern}'."
        );

        eprintln!(
            "To allow this request, add '{host_pattern}' to the allowed outbound hosts config."
        );

        return false;
    }
    if blocked_networks.is_blocked(&addr) {
        eprintln!("Destination IP is blocked: {}", addr);

        return false;
    }
    true
}

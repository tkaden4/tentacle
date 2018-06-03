use nix;
use nix::ifaddrs::getifaddrs;
use nix::sys::socket;
use nix::sys::socket::SockAddr;

use std::net::Ipv4Addr;

use serde::*;
use serde_json;

use message::Version;

pub const TENTACLE_PORT: u16 = 6406;
pub const TENTACLE_VERSION: Version = Version { major: 0, minor: 1, patch: 0 };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceInfo {
    pub name: String,
    pub address: Ipv4Addr,
    pub broadcast: Ipv4Addr
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Service {
    pub name: String,
    pub ip: Ipv4Addr
}

impl InterfaceInfo {
    pub fn new(name: String, address: Ipv4Addr, broadcast: Ipv4Addr) -> Self {
        Self {
            name: name,
            address: address,
            broadcast: broadcast
        }
    }
}

pub fn sockaddr_as_ip(addr: SockAddr) -> Option<Ipv4Addr> {
    match addr {
        SockAddr::Inet(inet) => {
            match inet.ip() {
                socket::IpAddr::V4(ipv4) => {
                    Some(ipv4.to_std())
                },
                _ => None
            }
        },
        _ => None
    }
}

pub fn interfaces() -> nix::Result<impl Iterator<Item=InterfaceInfo>> {
    let addrs = getifaddrs()?.filter_map(|ifaddr|{
        ifaddr.broadcast
            .and_then(sockaddr_as_ip)
            .and_then(|broad| {
                ifaddr.address
                    .and_then(sockaddr_as_ip)
                    .map(|addr| InterfaceInfo::new(ifaddr.interface_name, addr, broad))
            })
    });
    Ok(addrs)
}

use std::{net::SocketAddr, time::Duration, io, mem::MaybeUninit};

use socket2::{Domain, Socket as SystemSocket, Type, Protocol};
use crate::packet::ip::IpNextLevelProtocol;

use super::{SocketOption, IpVersion, SocketType};

pub fn check_socket_option(socket_option: SocketOption) -> Result<(), String> {
    match socket_option.ip_version {
        IpVersion::V4 => {
            match socket_option.socket_type {
                SocketType::Raw => {
                    match socket_option.protocol {
                        Some(IpNextLevelProtocol::Icmp) => Ok(()),
                        Some(IpNextLevelProtocol::Tcp) => Ok(()),
                        Some(IpNextLevelProtocol::Udp) => Ok(()),
                        _ => Err(String::from("Invalid protocol")),
                    }
                }
                SocketType::Dgram => {
                    match socket_option.protocol {
                        Some(IpNextLevelProtocol::Icmp) => Ok(()),
                        Some(IpNextLevelProtocol::Udp) => Ok(()),
                        _ => Err(String::from("Invalid protocol")),
                    }
                }
                SocketType::Stream => {
                    match socket_option.protocol {
                        Some(IpNextLevelProtocol::Tcp) => Ok(()),
                        _ => Err(String::from("Invalid protocol")),
                    }
                }
            }
        }
        IpVersion::V6 => {
            match socket_option.socket_type {
                SocketType::Raw => {
                    match socket_option.protocol {
                        Some(IpNextLevelProtocol::Icmpv6) => Ok(()),
                        Some(IpNextLevelProtocol::Tcp) => Ok(()),
                        Some(IpNextLevelProtocol::Udp) => Ok(()),
                        _ => Err(String::from("Invalid protocol")),
                    }
                }
                SocketType::Dgram => {
                    match socket_option.protocol {
                        Some(IpNextLevelProtocol::Icmpv6) => Ok(()),
                        Some(IpNextLevelProtocol::Udp) => Ok(()),
                        _ => Err(String::from("Invalid protocol")),
                    }
                }
                SocketType::Stream => {
                    match socket_option.protocol {
                        Some(IpNextLevelProtocol::Tcp) => Ok(()),
                        _ => Err(String::from("Invalid protocol")),
                    }
                }
            }
        }
    }
}

/// Receive all IPv4 or IPv6 packets passing through a network interface
pub struct ListenerSocket {
    inner: SystemSocket,
}

impl ListenerSocket {
    pub fn new(_socket_addr: SocketAddr, ip_version: IpVersion, protocol: Option<IpNextLevelProtocol>, timeout: Option<Duration>) -> io::Result<ListenerSocket> {
        let socket = match ip_version {
            IpVersion::V4 => {
                match protocol {
                    Some(IpNextLevelProtocol::Icmp) => SystemSocket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?,
                    Some(IpNextLevelProtocol::Tcp) => SystemSocket::new(Domain::IPV4, Type::RAW, Some(Protocol::TCP))?,
                    Some(IpNextLevelProtocol::Udp) => SystemSocket::new(Domain::IPV4, Type::RAW, Some(Protocol::UDP))?,
                    _ => SystemSocket::new(Domain::IPV4, Type::RAW, None)?,
                }
            },
            IpVersion::V6 => {
                match protocol {
                    Some(IpNextLevelProtocol::Icmpv6) => SystemSocket::new(Domain::IPV6, Type::RAW, Some(Protocol::ICMPV6))?,
                    Some(IpNextLevelProtocol::Tcp) => SystemSocket::new(Domain::IPV6, Type::RAW, Some(Protocol::TCP))?,
                    Some(IpNextLevelProtocol::Udp) => SystemSocket::new(Domain::IPV6, Type::RAW, Some(Protocol::UDP))?,
                    _ => SystemSocket::new(Domain::IPV6, Type::RAW, None)?,
                }
            },
        };
        if let Some(timeout) = timeout {
            socket.set_read_timeout(Some(timeout))?;
        }
        //socket.bind(&socket_addr.into())?;
        Ok(ListenerSocket {
            inner: socket,
        })
    }
    pub fn receive_from(&self, buf: &mut Vec<u8>) -> io::Result<(usize, SocketAddr)> {
        let recv_buf =
            unsafe { &mut *(buf.as_mut_slice() as *mut [u8] as *mut [MaybeUninit<u8>]) };
        loop {
            match self.inner.recv_from(recv_buf) {
                Ok((packet_len, addr)) => {
                    match addr.as_socket() {
                        Some(socket_addr) => {
                            return Ok((packet_len, socket_addr));
                        },
                        None => continue,
                    }
                },
                Err(_) => {
                    continue;
                },
            }
        }
    }
}

/// Module for datalink layer and network interface.
pub mod datalink;
/// Module for network packets
pub mod packet;
/// Module for packet capture
pub mod pcap;
/// Module for sockets
pub mod socket;

#[cfg(feature = "setup")]
extern crate cross_socket_deps;

#[cfg(feature = "setup")]
pub mod deps {
    pub use cross_socket_deps::*;
}

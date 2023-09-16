use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use default_net::Interface;
use cross_socket::option::PacketCaptureOptions;
use cross_socket::listener::Listner;
use cross_socket::packet::ip::IpNextLevelProtocol;

// Start capturing TCP packets on the default interface
// Filter: Protocol: TCP only, Ports: 22, 80, 443, 4433, 5000, 8080, 8443, 8888, 9000, 9443
// Stop after 10 seconds using the stop handle
fn main() {
    // Get default interface information
    let default_interface: Interface =
        default_net::get_default_interface().expect("Failed to get default interface information");
    // Filter: Protocol: TCP only, Ports: 22, 80, 443, 4433, 5000, 8080, 8443, 8888, 9000, 9443
    let capture_options: PacketCaptureOptions = PacketCaptureOptions {
        interface_index: default_interface.index,
        interface_name: default_interface.name,
        src_ips: HashSet::new(),
        dst_ips: HashSet::new(),
        src_ports: [22, 80, 443, 4433, 5000, 8080, 8443, 8888, 9000, 9443].iter().cloned().collect(),
        dst_ports: [22, 80, 443, 4433, 5000, 8080, 8443, 8888, 9000, 9443].iter().cloned().collect(),
        ether_types: HashSet::new(),
        ip_protocols: [IpNextLevelProtocol::Tcp].iter().cloned().collect(),
        duration: Duration::from_secs(30),
        promiscuous: false,
        store: false,
        store_limit: 0,
        receive_undefined: false,
    };
    // Create new listener
    let listener: Listner = Listner::new(capture_options);
    let rx = listener.get_receiver();
    let stop = listener.get_stop_handle();
    // Stop after 10 seconds
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        *stop.lock().unwrap() = true;
    });
    // Start capturing packets
    let handle = thread::spawn(move || listener.start());
    // Print captured packets
    while let Ok(msg) = rx.lock().unwrap().recv() {
        println!("{:?}", msg);
    }
    let _result = handle.join().unwrap();
}
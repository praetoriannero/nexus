use nexus::ethernet::Ethernet;
use nexus::ip::Ip;
use pcap::Capture;

fn main() {
    let mut cap = Capture::from_file("/home/nick/source/nexus/data/test.pcapng").unwrap();
    while let Ok(packet) = cap.next_packet() {
        println!("{:?}", packet.data);
        let mut eth_pdu = Ethernet::from_bytes(packet.data).unwrap();
        eth_pdu.set_ether_type(0);
        let _ip_pdu = Ip::from_bytes(eth_pdu.payload()).unwrap();
        // println!("\n{:?}", eth_pdu);
        // println!("{:?}", ip_pdu);
    }
}

use nexus::ethernet::Ethernet;
use nexus::ip::Ip;
use nexus::pdu::Pdu;
use pcap::Capture;

fn main() {
    let mut cap = Capture::from_file("/home/nick/source/nexus/data/test.pcapng").unwrap();
    while let Ok(packet) = cap.next_packet() {
        let eth_pdu = Ethernet::from_bytes(&packet.data).unwrap();
        let ip_pdu = Ip::from_bytes(&eth_pdu.data.unwrap()).unwrap();
        println!("\n{:?}", eth_pdu);
        println!("{:?}", ip_pdu);
    }
}

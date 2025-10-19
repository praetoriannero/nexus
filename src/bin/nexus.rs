use nexus::ethernet::Ethernet;
use nexus::ip::Ip;
use nexus::pdu::Deserialize;
use pcap::Capture;

fn main() {
    let mut cap = Capture::from_file("/home/nick/source/nexus/data/test.pcapng").unwrap();
    while let Ok(packet) = cap.next_packet() {
        let eth_pdu = Ethernet::from_bytes(&packet.data, None).unwrap();
        let ip_pdu = Ip::from_bytes(&eth_pdu.data.unwrap(), None).unwrap();
        println!("{:?}", eth_pdu);
        println!("{:?}", eth_pdu.dst_addr());
    }
}

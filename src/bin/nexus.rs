use chrono::DateTime;
use nexus::ethernet::Ethernet;
use nexus::ip::Ip;
use nexus::pdu::Pdu;
use nexus::utils::printable_ascii;
use pcap::Capture;

fn main() {
    let mut cap = Capture::from_file("/home/nick/source/nexus/data/test.pcapng").unwrap();
    while let Ok(packet) = cap.next_packet() {
        println!(
            "{:?} {}",
            DateTime::from_timestamp(packet.header.ts.tv_sec, packet.header.ts.tv_usec as u32)
                .unwrap(),
            printable_ascii(packet.data)
        );
        let bytes = packet.data.to_vec();
        let eth_pdu = Ethernet::from_bytes(&bytes).unwrap();
        let Some(inner) = eth_pdu.child_pdu() else {
            continue;
        };
        let ip_pdu = inner.downcast_ref::<Ip>().unwrap();
        println!("{}", ip_pdu.src_addr());
    }
}

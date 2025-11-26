use chrono::DateTime;
use nexus_lib::ethernet::Ethernet;
use nexus_lib::ip::Ip;
use nexus_lib::pdu::Pdu;
use nexus_lib::utils::printable_ascii;
use pcap::Capture;

fn main() {
    let mut cap = Capture::from_file("./data/test.pcapng").unwrap();
    while let Ok(packet) = cap.next_packet() {
        println!(
            "{:?} {}",
            DateTime::from_timestamp(packet.header.ts.tv_sec, packet.header.ts.tv_usec as u32)
                .unwrap(),
            printable_ascii(packet.data)
        );
        let Ok(mut eth_pdu) = Ethernet::from_bytes(&packet.data) else {
            continue;
        };
        let Some(inner) = eth_pdu.child_pdu() else {
            continue;
        };
        let mut ip_pdu = inner.downcast_mut::<Ip>().unwrap();
        println!("{}", ip_pdu.src_addr());
    }
}

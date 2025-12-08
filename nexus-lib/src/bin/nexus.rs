use chrono::DateTime;
use nexus_lib::ethernet::Ethernet;
use nexus_lib::ip::Ip;
use nexus_lib::pdu::{Pdu, deserialize};
use nexus_lib::utils::printable_ascii;
use pcap::Capture;

fn main() {
    let mut cap = Capture::from_file("./data/test.pcapng").unwrap();
    let mut index = 0;
    while let Ok(packet) = cap.next_packet() {
        index += 1;
        println!(
            "{} {:?} {}",
            index,
            DateTime::from_timestamp(
                packet.header.ts.tv_sec,
                (packet.header.ts.tv_usec * 1_000) as u32
            )
            .unwrap(),
            printable_ascii(packet.data)
        );

        let Ok(eth_pdu) = Ethernet::from_bytes(&packet.data) else {
            continue;
        };

        //
        // if let Some(mut eth_pdu3) = deserialize::<Ethernet>(&packet.data) {
        //     println!("Old3 ether type {}", eth_pdu3.ether_type());
        //     eth_pdu3.set_ether_type(1);
        //     println!("New3 ether type {}", eth_pdu3.ether_type());
        // }
        //
        // if let Some(eth_pdu2) = eth_pdu.find_mut::<Ethernet>() {
        //     println!("Old2 ether type {}", eth_pdu2.ether_type());
        //     eth_pdu2.set_ether_type(1);
        //     println!("New2 ether type {}", eth_pdu2.ether_type());
        // }

        println!(
            "{}",
            serde_json::to_string_pretty(&eth_pdu.to_json().unwrap()).unwrap()
        );
        //
        // if let Some(ip_pdu) = eth_pdu.find::<Ip>() {
        //     println!("{}", ip_pdu.src_addr());
        // } else {
        //     continue;
        // };
        // let Some(inner) = eth_pdu.child_pdu_mut() else {
        //     continue;
        // };
        //
        // let ip_pdu = inner.downcast_mut::<Ip>().unwrap();
        // println!("{}", ip_pdu.src_addr());
    }
}

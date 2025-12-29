use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct MacAddr(pub [u8; 6]);

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct EthernetII {
    dst_addr: MacAddr,
    src_addr: MacAddr,
    #[deku(endian = "big")]
    eth_type: u16,
}

fn main() {
    let ethernet_frame: [u8; 56] = [
        // Destination MAC: 00:11:22:33:44:55
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, // Source MAC: 66:77:88:99:aa:bb
        0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, // EtherType: IPv4 (0x0800)
        0x08, 0x00,
        // Payload (46 bytes minimum)
        // Fake IPv4 header start (not fully valid, but structurally realistic)
        0x45, 0x00, 0x00, 0x2e, // Version/IHL, TOS, Total Length
        0x12, 0x34, 0x00, 0x00, // Identification, Flags/Fragment
        0x40, 0x11, 0x00, 0x00, // TTL, Protocol (UDP), Header checksum
        0xc0, 0xa8, 0x01, 0x01, // Src IP: 192.168.1.1
        0xc0, 0xa8, 0x01, 0x02, // Dst IP: 192.168.1.2
        // Remaining payload padding
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let ((bytes_remaining, bit_offset), mut eth_pdu) =
        EthernetII::from_bytes((&ethernet_frame, 0)).unwrap();
    eth_pdu.eth_type = 65535;
    println!("{:?}", bytes_remaining);
    println!("{:?}", bit_offset);
    println!("{:?}", eth_pdu);
    println!("{:?}", ethernet_frame);
    println!("{:?}", eth_pdu.to_bytes().unwrap());
}

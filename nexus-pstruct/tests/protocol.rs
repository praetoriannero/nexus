use nexus_pstruct::Protocol;

//  #[derive(Protocol)]
//  struct Ipv4 {
//      #[field]
//      version: u4,
//
//      #[field]
//      ihl: u4,
//
//      #[field]
//      total_len: u16,
//
//      #[field]
//      identification: u16,
//
//      #[field(hidden=true)]
//      res: u1,
//
//      #[field]
//      df: u1,
//
//      #[field]
//      mf: u1,
//
//      #[field]
//      frag_offset: u13,
//
//      #[field]
//      ttl: u8,
//
//      #[field]
//      protocol: u8,
//
//      #[field]
//      checksum: u16,
//
//      #[field]
//      src_addr: u32,
//
//      #[field]
//      dst_addr: u32,
//  }

//  #[field(
//      skip: bool,
//      pad_right: usize,
//      pad_left: usize,
//      count: fn(&[u8]) -> usize,
//      enable: fn(&[u8]) -> bool,
//      disable: fn(&[u8]) -> bool,
//  )]

#[test]
fn integ() {
    #![allow(dead_code)]

    use arbitrary_int::prelude::*;
    use bit_ext::prelude::*;
    #[derive(Protocol, Default)]
    struct Ipv4 {
        #[field]
        version: u4,
        #[field]
        ihl: u4,
    }

    let _ipv4_tcp_hello: [u8; 45] = [
        // IPv4 header (20 bytes)
        0x45, 0x3c, // Version/IHL, DSCP/ECN
        0x00, 0x2D, // Total Length = 45 bytes
        0x1C, 0x46, // Identification
        0x40, 0x00, // Flags (DF) + Fragment offset
        0x40, // TTL = 64
        0x06, // Protocol = TCP (6)
        0x32, 0x4E, // Header checksum (0x324E) -- correct for this header
        0xC0, 0x00, 0x02, 0x01, // Src IP: 192.0.2.1
        0xC6, 0x33, 0x64, 0x02, // Dst IP: 198.51.100.2
        // TCP header (20 bytes)
        0x30, 0x39, // Src port = 12345
        0x00, 0x50, // Dst port = 80
        0x01, 0x02, 0x03, 0x04, // Seq number
        0x00, 0x00, 0x00, 0x00, // Ack number
        0x50, 0x18, // Data offset (5) << 4 , Flags (PSH+ACK)
        0xFF, 0xFF, // Window size
        0x00, 0x00, // Checksum (left 0x0000 for test)
        0x00, 0x00, // Urgent pointer
        // Payload: "hello"
        0x68, 0x65, 0x6C, 0x6C, 0x6F,
    ];

    let p = Ipv4::default();
    println!("{}", p.version());
    println!("{}", p.ihl());
    println!("{:?}", p.marked_fields());
    println!("Ipv4::total_width() = {}", Ipv4::total_width());
}

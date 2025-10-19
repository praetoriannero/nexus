use crate::pdu::{Deserialize, Pdu, Serialize};
use crate::utils::{Endian, parse_bytes};
use std::net::Ipv4Addr;

pub static IPV4_BYTE_MULTIPLE: usize = 4;
static IPV4_VERSION_OFFSET: usize = 0;
static IPV4_TOS_OFFSET: usize = 1;
static IPV4_TOTAL_LEN_OFFSET: usize = 2;
static IPV4_ID_OFFSET: usize = 4;
static IPV4_FRAG_FLAG_OFFSET: usize = 6;
static IPV4_TTL_OFFSET: usize = 8;
static IPV4_PROTO_OFFSET: usize = 9;
static IPV4_CHECKSUM_OFFSET: usize = 10;
static IPV4_SRC_ADDR_OFFSET: usize = 12;
static IPV4_DST_ADDR_OFFSET: usize = 16;
static IPV4_OPT_OFFSET: usize = 20;
static IPV4_HEADER_LEN: usize = 20;

#[derive(Debug, Default)]
struct IpOptionHeader {
    opt: u32,
}

#[derive(Debug)]
pub struct IpOption<'a> {
    bytes: &'a [u8],
    pub pdu_chain: Option<Vec<&'a Pdu<'a>>>,
    header: IpOptionHeader,
}

impl<'a> IpOption<'a> {
    pub fn from_bytes(bytes: &'a [u8], pdu_chain: Option<Vec<&'a Pdu>>) -> Self {
        Self {
            bytes,
            pdu_chain,
            header: IpOptionHeader::default(),
        }
    }
}

#[derive(Debug, Default)]
struct IpHeader {
    vihl: u8,
    tos: u8,
    total_len: u16,
    id: u16,
    flag_frag: u16,
    ttl: u8,
    proto: u8,
    checksum: u16,
    src_addr: [u8; 4],
    dst_addr: [u8; 4],
}

#[derive(Debug)]
pub struct Ip<'a> {
    bytes: &'a [u8],
    header: IpHeader,
    opts: Vec<IpOption<'a>>,
    pub pdu_chain: Option<Vec<&'a Pdu<'a>>>,
    pub data: Option<&'a [u8]>,
    pub child_pdu: Option<&'a Pdu<'a>>,
    pub parent_pdu: Option<&'a Pdu<'a>>,
}

impl<'a> Serialize<'a> for Ip<'a> {
    fn finalize(&'a mut self) {
        // TODO: need to calculate checksum here
        ()
    }
}

impl<'a> Deserialize<'a> for Ip<'a> {
    fn from_bytes(bytes: &'a [u8], pdu_chain: Option<Vec<&'a Pdu>>) -> Option<Self> {
        if bytes.len() < IPV4_HEADER_LEN {
            return None;
        }

        Some(Self {
            bytes: &bytes[..std::mem::size_of::<IpHeader>()],
            pdu_chain,
            opts: Vec::new(),
            data: Some(&bytes[std::mem::size_of::<IpHeader>()..]),
            header: IpHeader::default(),
            child_pdu: None,
            parent_pdu: None,
        })
    }
}

impl<'a> Ip<'a> {
    pub fn version(&self) -> u8 {
        (self.bytes[IPV4_VERSION_OFFSET] & 0xF0) >> 4
    }

    pub fn ihl(&self) -> u16 {
        let ihl = (self.bytes[IPV4_VERSION_OFFSET] & 0x0F) as usize * IPV4_BYTE_MULTIPLE;
        assert!(ihl >= IPV4_HEADER_LEN);
        ihl as u16
    }

    pub fn tos(&self) -> u8 {
        self.bytes[IPV4_TOS_OFFSET]
    }

    pub fn dscp(&self) -> u8 {
        self.tos() >> 4
    }

    pub fn ecn(&self) -> u8 {
        self.tos() & 0xF
    }

    pub fn total_len(&self) -> u16 {
        parse_bytes::<u16>(
            &self.bytes[IPV4_TOTAL_LEN_OFFSET..IPV4_ID_OFFSET],
            Endian::Big,
        )
    }

    pub fn id(&self) -> u16 {
        parse_bytes::<u16>(
            &self.bytes[IPV4_ID_OFFSET..IPV4_FRAG_FLAG_OFFSET],
            Endian::Big,
        )
    }

    pub fn flags(&self) -> u8 {
        self.bytes[IPV4_FRAG_FLAG_OFFSET] >> 5
    }

    pub fn rf(&self) -> bool {
        ((self.flags() >> 2) & 0b1) != 0
    }

    pub fn df(&self) -> bool {
        ((self.flags() >> 1) & 0b1) != 0
    }

    pub fn mf(&self) -> bool {
        (self.flags() & 0x1) != 0
    }

    pub fn frag_offset(&self) -> u16 {
        parse_bytes::<u16>(
            &self.bytes[IPV4_FRAG_FLAG_OFFSET..IPV4_TTL_OFFSET],
            Endian::Big,
        ) & 0x1FFF
    }

    pub fn ttl(&self) -> u8 {
        self.bytes[IPV4_TTL_OFFSET]
    }

    pub fn protocol(&self) -> u8 {
        self.bytes[IPV4_PROTO_OFFSET]
    }

    pub fn checksum(&self) -> u16 {
        parse_bytes::<u16>(
            &self.bytes[IPV4_CHECKSUM_OFFSET..IPV4_SRC_ADDR_OFFSET],
            Endian::Big,
        )
    }

    pub fn src_addr(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(parse_bytes(
            &self.bytes[IPV4_SRC_ADDR_OFFSET..IPV4_DST_ADDR_OFFSET],
            Endian::Big,
        ))
    }

    pub fn dst_addr(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(parse_bytes(
            &self.bytes[IPV4_DST_ADDR_OFFSET..IPV4_OPT_OFFSET],
            Endian::Big,
        ))
    }

    // pub fn new(
    //     version: u8,
    //     ihl: u8,
    //     tos: u8,
    //     total_len: u16,
    //     id: u16,
    //     flags: u8,
    //     frag_offset: u16,
    //     ttl: u8,
    //     protocol: u8,
    //     src_addr: u32,
    //     dst_addr: u32,
    //     options: Option<IpOption>,
    //     payload: Vec<u8>,
    // ) -> Self {
    //     let vihl = (version << 4) | ihl;
    //     buffer.push(vihl);
    //     buffer.push(tos);
    //     for idx in (0..2).rev() {
    //         let temp = total_len >> (8 * idx);
    //         buffer.push(temp as u8);
    //     }
    //     for idx in (0..2).rev() {
    //         let temp = id >> (8 * idx);
    //         buffer.push(temp as u8);
    //     }
    //     buffer.push(flags);
    //     for idx in (0..2).rev() {
    //         let temp = frag_offset >> (8 * idx);
    //         buffer.push(temp as u8);
    //     }
    //     buffer.push(ttl);
    //     buffer.push(protocol);
    //     for _ in 0..2 {
    //         // checksum
    //         buffer.push(0);
    //     }
    //     for idx in (0..4).rev() {
    //         let temp = src_addr >> (8 * idx);
    //         buffer.push(temp as u8);
    //     }
    //     for idx in (0..4).rev() {
    //         let temp = dst_addr >> (8 * idx);
    //         buffer.push(temp as u8);
    //     }
    //     match options {
    //         // TODO: do something with the options
    //         Some(_) => (),
    //         None => (),
    //     }
    //     buffer.extend(payload);
    //     Self {
    //         bytes: buffer,
    //         // TODO: add pdu_chain
    //         pdu_chain: None,
    //         child_pdu: None,
    //         data: &payload,
    //         parent_pdu: None,
    //     }
    // }
    // pub fn set_version(&'a mut self, version: u8) -> &'a Self {
    //     self
    // }
    //
    // pub fn set_ihl(&'a mut self, ihl: u8) -> &'a Self {
    //     self
    // }
    //
    // pub fn set_tos(&'a mut self, tos: u8) -> &'a Self {
    //     self
    // }
    //
}

#[cfg(test)]
mod tests {
    use super::*;

    static IPV4_TCP_HELLO: [u8; 45] = [
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
    #[test]
    fn test_ip_from_bytes() {
        Ip::from_bytes(&IPV4_TCP_HELLO, None);
    }

    #[test]
    fn test_ip_get_version() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes, None).unwrap();
        assert!(ip_pdu.version() == 4);
    }

    #[test]
    fn test_ip_get_ihl() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes, None).unwrap();
        assert!(IPV4_HEADER_LEN == ip_pdu.ihl() as usize);
    }

    #[test]
    fn test_ip_get_tos() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes, None).unwrap();
        assert!(ip_pdu.tos() == 0x3c);
    }

    #[test]
    fn test_ip_get_dscp() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes, None).unwrap();
        assert!(ip_pdu.dscp() == 0x3);
    }

    #[test]
    fn test_ip_get_ecn() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes, None).unwrap();
        assert!(ip_pdu.ecn() == 0xc);
    }
}

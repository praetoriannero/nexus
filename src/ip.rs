use crate::error::ParseError;
use crate::pdu::{Pdu, PduKind, PduType};
use crate::utils::{Endian, parse_bytes};
use std::borrow::Cow;
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

#[derive(Debug, Clone)]
pub struct IpOption<'a> {
    header: Cow<'a, [u8]>,
}

impl<'a> IpOption<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            header: Cow::Borrowed(&bytes),
        }
    }
}

pub struct Ip<'a> {
    header: Cow<'a, [u8]>,
    opts: Vec<IpOption<'a>>,
    data: Cow<'a, [u8]>,
    parent: Option<Box<dyn Pdu<'a>>>,
    child: Option<Box<dyn Pdu<'a>>>,
}

fn get_ip_header_len<'a>(ip_header_bytes: &'a [u8]) -> usize {
    (ip_header_bytes[IPV4_VERSION_OFFSET] & 0xF) as usize * IPV4_BYTE_MULTIPLE
}

impl<'a> Pdu<'a> for Ip<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        vec![0; IPV4_HEADER_LEN as usize]
    }

    fn pdu_type(&self) -> PduType {
        PduType::Ip
    }

    fn parent_pdu(&self) -> &Option<Box<dyn Pdu<'a>>> {
        &self.parent
    }

    fn child_pdu(&self) -> &Option<Box<dyn Pdu<'a>>> {
        &self.child
    }

    fn pdu_kind(&self) -> crate::pdu::PduKind {
        PduKind(Self::_kind)
    }

    fn static_pdu_kind() -> crate::pdu::PduKind {
        PduKind(Ip::_kind)
    }
}

impl<'a> Ip<'a> {
    fn _kind() {}

    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError> {
        if bytes.len() < IPV4_HEADER_LEN {
            return Err(ParseError::NotEnoughData);
        }

        let header_len = get_ip_header_len(&bytes[..IPV4_HEADER_LEN]);
        if header_len > bytes.len() {
            return Err(ParseError::NotEnoughData);
        }

        let result = Self {
            opts: Vec::new(),
            data: Cow::Borrowed(&bytes[header_len..]),
            header: Cow::Borrowed(&bytes[..header_len]),
            child: None,
            parent: None,
        };

        Ok(result)
    }

    pub fn new() -> Self {
        Self {
            opts: Vec::new(),
            header: Cow::Owned(Vec::with_capacity(IPV4_HEADER_LEN)),
            data: Cow::Owned(Vec::new()),
            child: None,
            parent: None,
        }
    }

    pub fn version(&self) -> u8 {
        (self.header[IPV4_VERSION_OFFSET] & 0xF0) >> 4
    }

    pub fn set_version(&mut self, version: u8) {
        let version_byte = &mut self.header.to_mut()[IPV4_VERSION_OFFSET];
        (*version_byte) &= 0x0F;
        (*version_byte) &= version << 4;
    }

    pub fn with_version(&mut self, version: u8) -> &mut Self {
        self.set_version(version);
        self
    }

    pub fn ihl(&self) -> u16 {
        let ihl = (self.header[IPV4_VERSION_OFFSET] & 0x0F) as usize * IPV4_BYTE_MULTIPLE;
        assert!(ihl >= IPV4_HEADER_LEN);
        ihl as u16
    }

    pub fn set_ihl(&self, ihl: u16) {
        // TO-DO
    }

    pub fn with_ihl(&mut self, ihl: u16) -> &mut Self {
        self.set_ihl(ihl);
        self
    }

    pub fn tos(&self) -> u8 {
        self.header[IPV4_TOS_OFFSET]
    }

    pub fn set_tos(&self, tos: u8) {
        // TO-DO
    }

    pub fn with_tos(&mut self, tos: u8) -> &mut Self {
        self.set_tos(tos);
        self
    }

    pub fn dscp(&self) -> u8 {
        self.tos() >> 4
    }

    pub fn ecn(&self) -> u8 {
        self.tos() & 0xF
    }

    pub fn total_len(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[IPV4_TOTAL_LEN_OFFSET..IPV4_ID_OFFSET],
            Endian::Big,
        )
    }

    pub fn id(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[IPV4_ID_OFFSET..IPV4_FRAG_FLAG_OFFSET],
            Endian::Big,
        )
    }

    pub fn flags(&self) -> u8 {
        self.header[IPV4_FRAG_FLAG_OFFSET] >> 5
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
            &self.header[IPV4_FRAG_FLAG_OFFSET..IPV4_TTL_OFFSET],
            Endian::Big,
        ) & 0x1FFF
    }

    pub fn ttl(&self) -> u8 {
        self.header[IPV4_TTL_OFFSET]
    }

    pub fn protocol(&self) -> u8 {
        self.header[IPV4_PROTO_OFFSET]
    }

    pub fn checksum(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[IPV4_CHECKSUM_OFFSET..IPV4_SRC_ADDR_OFFSET],
            Endian::Big,
        )
    }

    pub fn src_addr(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(parse_bytes(
            &self.header[IPV4_SRC_ADDR_OFFSET..IPV4_DST_ADDR_OFFSET],
            Endian::Big,
        ))
    }

    pub fn dst_addr(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(parse_bytes(
            &self.header[IPV4_DST_ADDR_OFFSET..IPV4_OPT_OFFSET],
            Endian::Big,
        ))
    }

    pub fn payload(&self) -> &[u8] {
        &self.data
    }

    pub fn set_payload(&mut self, payload: &[u8]) {
        self.data.to_mut().copy_from_slice(payload);
    }

    pub fn with_payload(&mut self, payload: &[u8]) -> &mut Self {
        self.set_payload(payload);
        self
    }
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
        Ip::from_bytes(&IPV4_TCP_HELLO).unwrap();
    }

    #[test]
    fn test_ip_get_version() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes).unwrap();
        assert!(ip_pdu.version() == 4);
    }

    #[test]
    fn test_ip_get_ihl() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes).unwrap();
        assert!(IPV4_HEADER_LEN == ip_pdu.ihl() as usize);
    }

    #[test]
    fn test_ip_get_tos() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes).unwrap();
        assert!(ip_pdu.tos() == 0x3c);
    }

    #[test]
    fn test_ip_get_dscp() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes).unwrap();
        assert!(ip_pdu.dscp() == 0x3);
    }

    #[test]
    fn test_ip_get_ecn() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes).unwrap();
        assert!(ip_pdu.ecn() == 0xc);
    }
}

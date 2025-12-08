use crate::error::ParseError;
use crate::ethernet::{ETHER_DISSECTION_TABLE, EtherType};
use crate::ip_opt::IpOption;
use crate::pdu::{Pdu, PduBuilder, PduResult, Pob, pdu_trait_assert};
use crate::register_eth_type;
use crate::utils::{Endian, parse_bytes};

use ctor::ctor;
use nexus_macros::{Tid, pdu_impl, pdu_type};
use nexus_tid::Tid;
use paste::paste;
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::{LazyLock, RwLock};

pub const IPV4_BYTE_MULTIPLE: usize = 4;
const IPV4_VERSION_OFFSET: usize = 0;
const IPV4_TOS_OFFSET: usize = 1;
const IPV4_TOTAL_LEN_OFFSET: usize = 2;
const IPV4_ID_OFFSET: usize = 4;
const IPV4_FRAG_FLAG_OFFSET: usize = 6;
const IPV4_TTL_OFFSET: usize = 8;
const IPV4_PROTO_OFFSET: usize = 9;
const IPV4_CHECKSUM_OFFSET: usize = 10;
const IPV4_SRC_ADDR_OFFSET: usize = 12;
const IPV4_DST_ADDR_OFFSET: usize = 16;
const IPV4_OPT_OFFSET: usize = 20;
const IPV4_HEADER_LEN: usize = 20;

fn get_ip_header_len<'a>(ip_header_bytes: &'a [u8]) -> usize {
    (ip_header_bytes[IPV4_VERSION_OFFSET] & 0xF) as usize * IPV4_BYTE_MULTIPLE
}

#[pdu_type]
pub struct Ip<'a> {
    opts: Vec<IpOption<'a>>,
}

#[pdu_impl]
impl<'a> Pdu<'a> for Ip<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = vec![0; IPV4_HEADER_LEN as usize];
        for idx in 0..self.opts.len() {
            res.extend_from_slice(&self.opts[idx].to_bytes());
        }
        res
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        if bytes.len() < IPV4_HEADER_LEN {
            return Err(ParseError::NotEnoughData);
        }

        let header_len = get_ip_header_len(&bytes[..IPV4_HEADER_LEN]);
        if header_len > bytes.len() {
            return Err(ParseError::NotEnoughData);
        }

        // TODO: actually parse the options

        let result = Self {
            opts: Vec::new(),
            data: Cow::Borrowed(&bytes[header_len..]),
            header: Cow::Borrowed(&bytes[..header_len]),
            child: None,
            parent: None,
        };

        Ok(Box::new(result))
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        todo!()
    }
}

impl<'a> Ip<'a> {
    pub fn new() -> Self {
        Self {
            opts: Vec::new(),
            header: Cow::Owned(vec![0; IPV4_HEADER_LEN]),
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

    pub fn set_ihl(&mut self, ihl: u8) {
        let ihl_byte = &mut self.header.to_mut()[IPV4_VERSION_OFFSET];
        *ihl_byte &= 0xF0;
        *ihl_byte &= ihl;
    }

    pub fn with_ihl(&mut self, ihl: u8) -> &mut Self {
        self.set_ihl(ihl);
        self
    }

    pub fn tos(&self) -> u8 {
        self.header[IPV4_TOS_OFFSET]
    }

    pub fn set_tos(&mut self, tos: u8) {
        self.header.to_mut()[IPV4_TOS_OFFSET] = tos;
    }

    pub fn with_tos(&mut self, tos: u8) -> &mut Self {
        self.set_tos(tos);
        self
    }

    pub fn dscp(&self) -> u8 {
        self.tos() >> 2
    }

    pub fn set_dscp(&mut self, dscp: u8) {
        let tos = &mut self.header.to_mut()[IPV4_TOS_OFFSET];
        *tos &= 0x0000_0011;
        *tos &= dscp;
    }

    pub fn with_dscp(&mut self, dscp: u8) -> &mut Self {
        self.set_dscp(dscp);
        self
    }

    pub fn ecn(&self) -> u8 {
        self.tos() & 0b0000_0011
    }

    pub fn set_ecn(&mut self, ecn: u8) {
        let tos = &mut self.header.to_mut()[IPV4_TOS_OFFSET];
        *tos &= 0b1111_1100;
        *tos &= ecn;
    }

    pub fn with_ecn(&mut self, ecn: u8) -> &mut Self {
        self.set_ecn(ecn);
        self
    }

    pub fn total_len(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[IPV4_TOTAL_LEN_OFFSET..IPV4_ID_OFFSET],
            Endian::Big,
        )
    }

    pub fn set_total_len(&mut self, total_len: u16) {
        self.header.to_mut()[IPV4_TOTAL_LEN_OFFSET..IPV4_ID_OFFSET]
            .copy_from_slice(&total_len.to_be_bytes());
    }

    pub fn with_total_len(&mut self, total_len: u16) -> &mut Self {
        self.set_total_len(total_len);
        self
    }

    pub fn id(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[IPV4_ID_OFFSET..IPV4_FRAG_FLAG_OFFSET],
            Endian::Big,
        )
    }

    pub fn set_id(&mut self, id: u16) {
        self.header.to_mut()[IPV4_ID_OFFSET..IPV4_FRAG_FLAG_OFFSET]
            .copy_from_slice(&id.to_be_bytes());
    }

    pub fn with_id(&mut self, id: u16) -> &mut Self {
        self.set_id(id);
        self
    }

    pub fn flags(&self) -> u8 {
        self.header[IPV4_FRAG_FLAG_OFFSET] >> 5
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.header.to_mut()[IPV4_FRAG_FLAG_OFFSET..IPV4_FRAG_FLAG_OFFSET + 1]
            .copy_from_slice(&flags.to_be_bytes());
    }

    pub fn with_flags(&mut self, flags: u8) -> &mut Self {
        self.set_flags(flags);
        self
    }

    pub fn rf(&self) -> bool {
        ((self.flags() >> 2) & 0b1) != 0
    }

    pub fn set_rf(&mut self, rf: u8) {
        let flags_byte = &mut self.header.to_mut()[IPV4_FRAG_FLAG_OFFSET];
        *flags_byte &= 0b0111_1111;
        *flags_byte &= rf;
    }

    pub fn with_rf(&mut self, rf: u8) -> &mut Self {
        self.set_rf(rf);
        self
    }

    pub fn df(&self) -> bool {
        ((self.flags() >> 1) & 0b1) != 0
    }

    pub fn set_df(&mut self, df: u8) {
        let flags_byte = &mut self.header.to_mut()[IPV4_FRAG_FLAG_OFFSET];
        *flags_byte &= 0b1011_1111;
        *flags_byte &= df;
    }

    pub fn with_df(&mut self, df: u8) -> &mut Self {
        self.set_df(df);
        self
    }

    pub fn mf(&self) -> bool {
        (self.flags() & 0x1) != 0
    }

    pub fn set_mf(&mut self, mf: u8) {
        let flags_byte = &mut self.header.to_mut()[IPV4_FRAG_FLAG_OFFSET];
        *flags_byte &= 0b1101_1111;
        *flags_byte &= mf;
    }

    pub fn with_mf(&mut self, mf: u8) -> &mut Self {
        self.set_mf(mf);
        self
    }

    pub fn frag_offset(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[IPV4_FRAG_FLAG_OFFSET..IPV4_TTL_OFFSET],
            Endian::Big,
        ) & 0x1FFF
    }

    pub fn set_frag_offset(&mut self, offset: u16) {
        self.header.to_mut()[IPV4_FRAG_FLAG_OFFSET..IPV4_TTL_OFFSET]
            .copy_from_slice(&offset.to_be_bytes());
    }

    pub fn with_frag_offset(&mut self, offset: u16) -> &mut Self {
        self.set_frag_offset(offset);
        self
    }

    pub fn ttl(&self) -> u8 {
        self.header[IPV4_TTL_OFFSET]
    }

    pub fn set_ttl(&mut self, ttl: u8) {
        self.header.to_mut()[IPV4_TTL_OFFSET] = ttl;
    }

    pub fn with_ttl(&mut self, ttl: u8) -> &mut Self {
        self.set_ttl(ttl);
        self
    }

    pub fn protocol(&self) -> u8 {
        self.header[IPV4_PROTO_OFFSET]
    }

    pub fn set_protocol(&mut self, protocol: u8) {
        self.header.to_mut()[IPV4_PROTO_OFFSET] = protocol;
    }

    pub fn with_protocol(&mut self, protocol: u8) -> &mut Self {
        self.set_protocol(protocol);
        self
    }

    pub fn checksum(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[IPV4_CHECKSUM_OFFSET..IPV4_SRC_ADDR_OFFSET],
            Endian::Big,
        )
    }

    pub fn set_checksum(&mut self, checksum: u16) {
        self.header.to_mut()[IPV4_CHECKSUM_OFFSET..IPV4_SRC_ADDR_OFFSET]
            .copy_from_slice(&checksum.to_be_bytes());
    }

    pub fn with_checksum(&mut self, checksum: u16) -> &mut Self {
        self.set_checksum(checksum);
        self
    }

    pub fn src_addr(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(parse_bytes(
            &self.header[IPV4_SRC_ADDR_OFFSET..IPV4_DST_ADDR_OFFSET],
            Endian::Big,
        ))
    }

    pub fn set_src_addr(&mut self, src_addr: Ipv4Addr) {
        self.header.to_mut()[IPV4_SRC_ADDR_OFFSET..IPV4_DST_ADDR_OFFSET]
            .copy_from_slice(&(src_addr.to_bits()).to_be_bytes());
    }

    pub fn with_src_addr(&mut self, src_addr: Ipv4Addr) -> &mut Self {
        self.set_src_addr(src_addr);
        self
    }

    pub fn dst_addr(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(parse_bytes(
            &self.header[IPV4_DST_ADDR_OFFSET..IPV4_OPT_OFFSET],
            Endian::Big,
        ))
    }

    pub fn set_dst_addr(&mut self, dst_addr: Ipv4Addr) {
        self.header.to_mut()[IPV4_DST_ADDR_OFFSET..IPV4_OPT_OFFSET]
            .copy_from_slice(&(dst_addr.to_bits()).to_be_bytes());
    }

    pub fn with_dst_addr(&mut self, dst_addr: Ipv4Addr) -> &mut Self {
        self.set_dst_addr(dst_addr);
        self
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

pub static IPV4_DISSECTION_TABLE: LazyLock<RwLock<HashMap<EtherType, PduBuilder>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[macro_export]
macro_rules! register_ipv4_type {
    ($ip_type:expr, $builder:ident) => {
        paste! {
            #[ctor]
            fn [<__nexus_register_ether_type_ $builder:lower>]() {
                pdu_trait_assert::<$builder>();
                if IPV4_DISSECTION_TABLE
                    .write()
                    .unwrap()
                    .insert($ip_type, |bytes: &'_ [u8]| -> PduResult<'_> {
                        $builder::from_bytes(bytes)
                    })
                    .is_some()
                {
                    panic!("IPv4 types can only be added once.")
                };
            }
        }
    };
}

register_eth_type!(EtherType(0x0800), Ip);

#[cfg(test)]
mod tests {
    use super::*;

    const IPV4_TCP_HELLO: [u8; 45] = [
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
        let ip_pdu = Ip::from_bytes(ip_bytes).unwrap().downcast::<Ip>().unwrap();
        assert!(ip_pdu.version() == 4);
    }

    #[test]
    fn test_ip_get_ihl() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes).unwrap().downcast::<Ip>().unwrap();
        assert!(IPV4_HEADER_LEN == ip_pdu.ihl() as usize);
    }

    #[test]
    fn test_ip_get_tos() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes).unwrap().downcast::<Ip>().unwrap();
        assert!(ip_pdu.tos() == 0x3c);
    }

    #[test]
    fn test_ip_get_dscp() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes).unwrap().downcast::<Ip>().unwrap();
        assert!(ip_pdu.dscp() == 0b0000_1111);
    }

    #[test]
    fn test_ip_get_ecn() {
        let ip_bytes = &IPV4_TCP_HELLO;
        let ip_pdu = Ip::from_bytes(ip_bytes).unwrap().downcast::<Ip>().unwrap();
        assert!(ip_pdu.ecn() == 0b0000_0000);
    }
}

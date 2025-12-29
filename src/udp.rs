use crate::ip::{IPV4_DISSECTION_TABLE, Ipv4Type};
use crate::ip6::{IPV6_DISSECTION_TABLE, Ipv6Type};
use crate::prelude::*;

const UDP_HEADER_LEN: usize = 8;
const UDP_SPORT_OFFSET: usize = 0;
const UDP_DPORT_OFFSET: usize = 2;
const UDP_LENGTH_OFFSET: usize = 4;
const UDP_CHECKSUM_OFFSET: usize = 6;

#[pdu_type]
pub struct Udp<'a> {}

#[pdu_impl]
impl<'a> Pdu<'a> for Udp<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.header);
        res
    }

    default_pdu_clone!(Udp);

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        // TODO: add dissection table logic
        Ok(Box::new(Self {
            header: Cow::Borrowed(&bytes[..UDP_HEADER_LEN]),
            parent: None,
            child: None,
        }))
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        Ok(json!({
            "udp": {
                "udp.sport": self.src_port(),
                "udp.dport": self.dst_port(),
                "udp.length": self.length(),
                "udp.checksum": self.checksum(),
                "udp.data": self.child_to_json(),
            }
        }))
    }
}

impl<'a> Udp<'a> {
    pub fn src_port(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[UDP_SPORT_OFFSET..UDP_DPORT_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_src_port(&mut self, sport: u16) {
        self.header.to_mut()[UDP_SPORT_OFFSET..UDP_DPORT_OFFSET]
            .copy_from_slice(&sport.to_be_bytes());
    }

    pub fn with_src_port(&mut self, sport: u16) -> &mut Self {
        self.set_src_port(sport);
        self
    }

    pub fn dst_port(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[UDP_DPORT_OFFSET..UDP_LENGTH_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_dst_port(&mut self, dport: u16) {
        self.header.to_mut()[UDP_DPORT_OFFSET..UDP_LENGTH_OFFSET]
            .copy_from_slice(&dport.to_be_bytes());
    }

    pub fn with_dst_port(&mut self, dport: u16) -> &mut Self {
        self.set_dst_port(dport);
        self
    }

    pub fn length(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[UDP_LENGTH_OFFSET..UDP_CHECKSUM_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_length(&mut self, length: u16) {
        self.header.to_mut()[UDP_LENGTH_OFFSET..UDP_CHECKSUM_OFFSET]
            .copy_from_slice(&length.to_be_bytes());
    }

    pub fn with_length(&mut self, length: u16) -> &mut Self {
        self.set_length(length);
        self
    }

    pub fn checksum(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[UDP_CHECKSUM_OFFSET..UDP_HEADER_LEN],
            crate::utils::Endian::Big,
        )
    }

    /// Set checkum to None if you want it to be autocalculated
    pub fn set_checksum(&mut self, checksum: Option<u16>) {
        if let Some(chksum) = checksum {
            self.header.to_mut()[UDP_CHECKSUM_OFFSET..UDP_HEADER_LEN]
                .copy_from_slice(&chksum.to_be_bytes());
        } else {
            ()
        }
    }

    pub fn with_checksum(&mut self, checksum: Option<u16>) -> &mut Self {
        self.set_checksum(checksum);
        self
    }
}

register_pdu!(Ipv4Type(0x11), Udp, IPV4_DISSECTION_TABLE);
register_pdu!(Ipv6Type(0x11), Udp, IPV6_DISSECTION_TABLE);

pub struct UdpType(pub u16);

pub static UDP_DISSECTION_TABLE: DissectionTable<UdpType> = create_table();

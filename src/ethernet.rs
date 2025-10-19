use crate::pdu::{Deserialize, Pdu, Serialize};
use crate::utils::parse_bytes;

static ETH_DST_OFFSET: usize = 0;
static ETH_SRC_OFFSET: usize = 6;
static ETH_TYPE_OFFSET: usize = 12;
static ETH_HEADER_LEN: usize = 14;

#[derive(Debug, Clone, Copy)]
pub struct MacAddress {
    address: [u8; 6],
}

impl MacAddress {
    pub fn from_bytes(bytes: [u8; 6]) -> Self {
        Self { address: bytes }
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        self.address.clone()
    }
}

#[derive(Debug, Clone, Copy)]
struct EthernetHeader {
    pub dst_addr: MacAddress,
    pub src_addr: MacAddress,
    pub ether_type: u16,
}

#[derive(Debug, Clone)]
pub struct Ethernet<'a> {
    bytes: Option<&'a [u8]>,
    header: Option<EthernetHeader>,
    pub pdu_chain: Option<Vec<&'a Pdu<'a>>>,
    pub data: Option<&'a [u8]>,
    pub child_pdu: Option<&'a Pdu<'a>>,
    pub parent_pdu: Option<&'a Pdu<'a>>,
}

impl<'a> Serialize<'a> for Ethernet<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(ETH_HEADER_LEN);
        bytes.extend(self.header.unwrap().clone().dst_addr.to_bytes());
        bytes
    }

    fn finalize(&'a mut self) {}
}

impl<'a> Deserialize<'a> for Ethernet<'a> {
    fn from_bytes(bytes: &'a [u8], pdu_chain: Option<Vec<&'a Pdu>>) -> Option<Self> {
        if bytes.len() < ETH_HEADER_LEN {
            return None;
        }

        Some(Self {
            bytes: Some(&bytes[..ETH_HEADER_LEN]),
            header: None,
            pdu_chain,
            data: Some(&bytes[ETH_HEADER_LEN..]),
            child_pdu: None,
            parent_pdu: None,
        })
    }
}

impl<'a> Ethernet<'a> {
    pub fn dst_addr(&self) -> &[u8] {
        &self.bytes.unwrap()[ETH_DST_OFFSET..ETH_SRC_OFFSET]
    }

    pub fn src_addr(&self) -> &[u8] {
        &self.bytes.unwrap()[ETH_SRC_OFFSET..ETH_TYPE_OFFSET]
    }

    pub fn ether_type(&self) -> u16 {
        parse_bytes::<u16>(
            &self.bytes.unwrap()[ETH_TYPE_OFFSET..ETH_HEADER_LEN],
            crate::utils::Endian::Little,
        )
    }

    pub fn new(dst_addr: MacAddress, src_addr: MacAddress, ether_type: u16) -> Self {
        Self {
            bytes: None,
            parent_pdu: None,
            child_pdu: None,
            pdu_chain: None,
            data: None,
            header: Some(EthernetHeader {
                dst_addr: dst_addr,
                src_addr: src_addr,
                ether_type: ether_type,
            }),
        }
    }
}

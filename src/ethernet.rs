use crate::error::ParseError;
use crate::pdu::Pdu;
use crate::utils::parse_bytes;
use std::mem;

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

impl EthernetHeader {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(mem::size_of::<EthernetHeader>());
        result.extend(self.dst_addr.to_bytes());
        result.extend(self.src_addr.to_bytes());
        result.extend(self.ether_type.to_be_bytes());
        result
    }
}

#[derive(Debug, Clone)]
pub struct Ethernet<'a> {
    bytes: Option<&'a [u8]>,
    header: Option<EthernetHeader>,
    pub data: Option<&'a [u8]>,
}

impl<'a> Pdu<'a> for Ethernet<'a> {
    fn to_bytes(&self) -> Result<Vec<u8>, ParseError> {
        match self.header {
            Some(header) => Ok(header.serialize()),
            None => Err(ParseError::InvalidHeader),
        }
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError> {
        if bytes.len() < ETH_HEADER_LEN {
            return Err(ParseError::NotEnoughData);
        }

        Ok(Self {
            bytes: Some(&bytes[..ETH_HEADER_LEN]),
            header: None,
            data: Some(&bytes[ETH_HEADER_LEN..]),
        })
    }
}

impl<'a> Ethernet<'a> {
    pub fn dst_addr(&self) -> &[u8] {
        &self.bytes.unwrap()[ETH_DST_OFFSET..ETH_SRC_OFFSET]
    }

    pub fn set_dst_addr(&mut self, dst_addr: MacAddress) {
        self.header.unwrap().dst_addr = dst_addr;
    }

    pub fn src_addr(&self) -> &[u8] {
        &self.bytes.unwrap()[ETH_SRC_OFFSET..ETH_TYPE_OFFSET]
    }

    pub fn set_src_addr(&mut self, src_addr: MacAddress) {
        self.header.unwrap().src_addr = src_addr;
    }

    pub fn ether_type(&self) -> u16 {
        parse_bytes::<u16>(
            &self.bytes.unwrap()[ETH_TYPE_OFFSET..ETH_HEADER_LEN],
            crate::utils::Endian::Little,
        )
    }

    pub fn set_ether_type(&mut self, ether_type: u16) {
        self.header.unwrap().ether_type = ether_type;
    }

    pub fn new(
        dst_addr: MacAddress,
        src_addr: MacAddress,
        ether_type: u16,
        data: Option<&'a [u8]>,
    ) -> Self {
        Self {
            bytes: None,
            data,
            header: Some(EthernetHeader {
                dst_addr: dst_addr,
                src_addr: src_addr,
                ether_type: ether_type,
            }),
        }
    }
}

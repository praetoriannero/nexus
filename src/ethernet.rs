use crate::error::ParseError;
use crate::pdu::{Pdu, PduKind, PduType, Pob};
use crate::utils::parse_bytes;

use num_enum::TryFromPrimitive;
use std::borrow::Cow;
use std::convert::TryFrom;

const ETH_DST_OFFSET: usize = 0;
const ETH_SRC_OFFSET: usize = 6;
const ETH_TYPE_OFFSET: usize = 12;
const ETH_HEADER_LEN: usize = 14;
const MAC_ADDR_SIZE: usize = 6;

#[derive(Debug, Clone, Copy)]
pub struct MacAddress<'a> {
    address: &'a [u8; MAC_ADDR_SIZE],
}

impl<'a> MacAddress<'a> {
    pub fn into_buff(&self, buff: &mut [u8]) -> Result<(), std::array::TryFromSliceError> {
        self.address.clone_into(buff.try_into()?);
        Ok(())
    }

    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            address: bytes.try_into().expect("err"),
        }
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        self.address.clone()
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum EtherType {
    Ipv4 = 0x0800,
    Arp = 0x0806,
    Ipv6 = 0x86DD,
}

fn pdu_from_type(ether_type: u16, bytes: &[u8]) -> Pob {
    let et = EtherType::try_from(ether_type).unwrap();
    match et {
        EtherType::Ipv4 => Some(Box::new((Ip::from_bytes(&bytes)).unwrap())),
        _ => None,
    }
}

fn get_ether_type(bytes: &[u8]) -> u16 {
    parse_bytes::<u16>(
        &bytes[ETH_TYPE_OFFSET..ETH_HEADER_LEN],
        crate::utils::Endian::Little,
    )
}

pub struct Ethernet<'a> {
    header: Cow<'a, [u8]>,
    data: Cow<'a, [u8]>,
    parent: Pob<'a>,
    child: Pob<'a>,
}

impl<'a> Pdu<'a> for Ethernet<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.header);
        res.extend_from_slice(&self.data);
        res
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError> {
        if bytes.len() < ETH_HEADER_LEN {
            return Err(ParseError::NotEnoughData);
        }

        let et = get_ether_type(bytes);
        let _inner = pdu_from_type(et, &bytes[ETH_HEADER_LEN..]);

        let Some(inner) = _inner else {
            return Err(ParseError::UnsupportedProtocol);
        };

        Ok(Self {
            header: Cow::Borrowed(&bytes[..ETH_HEADER_LEN]),
            data: Cow::Borrowed(&bytes[ETH_HEADER_LEN..]),
            parent: None,
            child: Some(inner),
        })
    }

    fn pdu_type(&self) -> PduType {
        PduType::Ethernet
    }

    fn parent_pdu(&self) -> &Pob<'a> {
        &self.parent
    }

    fn child_pdu(&self) -> &Pob<'a> {
        &self.child
    }

    fn dyn_pdu_kind(&self) -> PduKind {
        PduKind(Self::_kind)
    }

    fn static_pdu_kind() -> PduKind {
        PduKind(Ethernet::_kind)
    }
}

use crate::ip::Ip;
impl<'a> Ethernet<'a> {
    fn _kind() {}
    pub fn new() -> Self {
        Self {
            header: Cow::Owned(vec![0; ETH_HEADER_LEN]),
            data: Cow::Owned(Vec::new()),
            parent: None,
            child: None,
        }
    }

    pub fn with_dst_addr(&mut self, dst_addr: MacAddress) -> &mut Self {
        self.set_dst_addr(dst_addr);
        self
    }

    pub fn set_dst_addr(&mut self, dst_addr: MacAddress) {
        dst_addr
            .into_buff(&mut self.header.to_mut()[..ETH_DST_OFFSET])
            .unwrap();
    }

    pub fn dst_addr(&self) -> MacAddress {
        MacAddress::from_bytes(&self.header[ETH_DST_OFFSET..ETH_SRC_OFFSET])
    }

    pub fn with_src_addr(&mut self, src_addr: MacAddress) -> &mut Self {
        self.set_src_addr(src_addr);
        self
    }

    pub fn set_src_addr(&mut self, src_addr: MacAddress) {
        src_addr
            .into_buff(&mut self.header.to_mut()[ETH_DST_OFFSET..ETH_SRC_OFFSET])
            .unwrap();
    }

    pub fn src_addr(&self) -> MacAddress {
        MacAddress::from_bytes(&self.header[ETH_SRC_OFFSET..ETH_TYPE_OFFSET])
    }

    pub fn with_ether_type(&mut self, ether_type: u16) -> &mut Self {
        self.set_ether_type(ether_type);
        self
    }

    pub fn set_ether_type(&mut self, ether_type: u16) {
        self.header.to_mut()[ETH_TYPE_OFFSET..ETH_HEADER_LEN]
            .copy_from_slice(&ether_type.to_be_bytes());
    }

    pub fn ether_type(&self) -> u16 {
        get_ether_type(&self.header)
    }

    pub fn payload(&self) -> &[u8] {
        &self.data
    }

    pub fn set_payload(&mut self, payload: &[u8]) {
        self.data.to_mut().copy_from_slice(payload);
    }
}

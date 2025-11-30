use crate::error::ParseError;
use crate::ip::Ip;
use crate::mac_address::MacAddress;
use crate::pdu::{Pdu, Pob};
use crate::raw::Raw;
use crate::utils::{parse_bytes, printable_ascii};

use nexus_macros::{Tid, pdu_impl, pdu_type};
use nexus_tid::Tid;
use num_enum::TryFromPrimitive;
use serde::Serialize;
use serde::ser::SerializeStruct;
use std::any::TypeId;
use std::borrow::Cow;
use std::convert::TryFrom;

const ETH_DST_OFFSET: usize = 0;
const ETH_SRC_OFFSET: usize = 6;
const ETH_TYPE_OFFSET: usize = 12;
const ETH_HEADER_LEN: usize = 14;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum EtherType {
    Ipv4 = 0x0800,
    Arp = 0x0806,
    Ipv6 = 0x86DD,
}

fn pdu_from_type<'a>(ether_type: u16, bytes: &'a [u8]) -> Pob<'a> {
    let Ok(et) = EtherType::try_from(ether_type) else {
        return Some(Box::new(
            Raw::from_bytes(bytes).expect("Failed to create Raw PDU type"),
        ));
    };
    match et {
        EtherType::Ipv4 => Some(Box::new(
            Ip::from_bytes(bytes).expect("Failed to create IPv4 PDU type"),
        )),
        _ => Some(Box::new(
            Raw::from_bytes(bytes).expect("Failed to create Raw PDU type"),
        )),
    }
}

fn get_ether_type(bytes: &[u8]) -> u16 {
    parse_bytes::<u16>(
        &bytes[ETH_TYPE_OFFSET..ETH_HEADER_LEN],
        crate::utils::Endian::Big,
    )
}

#[pdu_type]
pub struct Ethernet<'a> {}

#[pdu_impl]
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

        let Some(inner) = pdu_from_type(get_ether_type(bytes), &bytes[ETH_HEADER_LEN..]) else {
            return Err(ParseError::UnsupportedProtocol);
        };

        Ok(Self {
            header: Cow::Borrowed(&bytes[..ETH_HEADER_LEN]),
            data: Cow::Borrowed(&bytes[ETH_HEADER_LEN..]),
            parent: None,
            child: Some(inner),
        })
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        todo!()
    }
}

impl<'a> Ethernet<'a> {
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
            .expect("Failed to set destination MAC address");
    }

    pub fn dst_addr(&self) -> MacAddress<'_> {
        MacAddress::from_bytes(&self.header[ETH_DST_OFFSET..ETH_SRC_OFFSET])
    }

    pub fn with_src_addr(&mut self, src_addr: MacAddress) -> &mut Self {
        self.set_src_addr(src_addr);
        self
    }

    pub fn set_src_addr(&mut self, src_addr: MacAddress) {
        src_addr
            .into_buff(&mut self.header.to_mut()[ETH_DST_OFFSET..ETH_SRC_OFFSET])
            .expect("Failed to set source MAC address");
    }

    pub fn src_addr(&self) -> MacAddress<'_> {
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

impl Serialize for Ethernet<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Ethernet", 4)?;
        s.serialize_field("eth.dst", &self.dst_addr().to_str())?;
        s.serialize_field("eth.src", &self.src_addr().to_str())?;
        s.serialize_field("eth.type", &self.ether_type())?;
        s.serialize_field("eth.data", &printable_ascii(self.payload()))?;
        s.end()
    }
}

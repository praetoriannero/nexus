use crate::error::ParseError;
use crate::pdu::Pdu;
use crate::utils::parse_bytes;

use std::borrow::Cow;

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

#[derive(Debug)]
pub struct Ethernet<'a> {
    header: Cow<'a, [u8]>,
    data: Cow<'a, [u8]>,
}

impl<'a> Pdu<'a> for Ethernet<'a> {
    fn to_bytes(&self) -> Result<Vec<u8>, ParseError> {
        Ok(Vec::new())
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError> {
        if bytes.len() < ETH_HEADER_LEN {
            return Err(ParseError::NotEnoughData);
        }

        Ok(Self {
            header: Cow::Borrowed(&bytes[..ETH_HEADER_LEN]),
            data: Cow::Borrowed(&bytes[ETH_HEADER_LEN..]),
        })
    }
}

impl<'a> Ethernet<'a> {
    pub fn new() -> Self {
        Self {
            header: Cow::Owned(Vec::with_capacity(ETH_HEADER_LEN)),
            data: Cow::Owned(Vec::new()),
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
        let _ = &self.header.to_mut()[..std::mem::size_of::<u16>()]
            .copy_from_slice(&ether_type.to_be_bytes());
    }

    pub fn ether_type(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[ETH_TYPE_OFFSET..ETH_HEADER_LEN],
            crate::utils::Endian::Little,
        )
    }
    pub fn payload(&self) -> &[u8] {
        &self.data
    }

    pub fn set_payload(&mut self, payload: &[u8]) {
        self.data.to_mut().copy_from_slice(payload);
    }
}

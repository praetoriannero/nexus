use crate::arena::Arena;
use crate::error::{AllocError, ParseError};
use crate::pdu::Pdu;
use crate::utils::parse_bytes;

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
    pub fn from_bytes_into(
        bytes: [u8; MAC_ADDR_SIZE],
        arena: &'a mut Arena,
    ) -> Result<Self, AllocError> {
        let Some(buff) = arena.alloc(MAC_ADDR_SIZE) else {
            return Err(AllocError::InsufficientSpace);
        };
        buff.clone_from_slice(&bytes[..]);
        Ok(Self {
            address: (*buff).try_into().expect("error during clone"),
        })
    }

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

    pub fn from_str(
        string: &str,
        arena: &'a mut Arena,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut temp: Vec<u8> = Vec::new();
        for oct in string.split(":") {
            temp.push(oct.parse()?);
        }
        MacAddress::from_bytes_into(temp.try_into().expect("error during clone"), arena)
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
    }

    pub fn new(addr: &'a [u8; 6]) -> Self {
        Self { address: addr }
    }
}

#[derive(Debug, Clone, Copy)]
struct EthernetHeader<'a> {
    bytes: &'a [u8],
}

impl<'a> EthernetHeader<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn new(
        dst_addr: MacAddress,
        src_addr: MacAddress,
        ether_type: u16,
        arena: &'a mut Arena,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Some(page) = &mut arena.alloc(ETH_HEADER_LEN) else {
            return Err(Box::new(AllocError::InsufficientSpace) as Box<dyn std::error::Error>);
        };
        // Instead of writing to page, we could write directly to arena
        dst_addr.into_buff(&mut page[..ETH_DST_OFFSET])?;
        src_addr.into_buff(&mut page[ETH_DST_OFFSET..ETH_SRC_OFFSET])?;
        page[ETH_SRC_OFFSET..ETH_HEADER_LEN].copy_from_slice(&ether_type.to_be_bytes());
        Ok(Self {
            // I think we want something like &arena[page.1..page.2]
            bytes: (*page).try_into().expect("err"),
        })
    }

    pub fn dst_addr(&self) -> MacAddress {
        MacAddress::from_bytes(&self.bytes[ETH_DST_OFFSET..ETH_SRC_OFFSET])
    }

    pub fn set_dst_addr(&mut self, _addr: &[u8]) -> &mut Self {
        self
    }

    pub fn src_addr(&self) -> MacAddress {
        MacAddress::from_bytes(&self.bytes[ETH_SRC_OFFSET..ETH_TYPE_OFFSET])
    }

    pub fn set_src_addr(&mut self, _addr: &[u8]) -> &mut Self {
        self
    }

    pub fn ether_type(&self) -> u16 {
        parse_bytes::<u16>(
            &self.bytes[ETH_TYPE_OFFSET..ETH_HEADER_LEN],
            crate::utils::Endian::Little,
        )
    }

    pub fn set_ether_type(&mut self, _ether_type: u16) -> &mut Self {
        self
    }
}

#[derive(Debug)]
pub struct Ethernet<'a> {
    header: EthernetHeader<'a>,
    pub data: &'a [u8],
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
            header: EthernetHeader::from_bytes(bytes),
            data: &bytes[ETH_HEADER_LEN..],
        })
    }
}

impl<'a> Ethernet<'a> {
    pub fn dst_addr(&self) -> MacAddress {
        self.header.dst_addr()
    }

    pub fn set_dst_addr(&mut self, _dst_addr: MacAddress) -> &mut Self {
        self
    }

    pub fn src_addr(&self) -> MacAddress {
        self.header.src_addr()
    }

    pub fn set_src_addr(&mut self, _src_addr: MacAddress) -> &mut Self {
        self
    }

    pub fn ether_type(&self) -> u16 {
        self.header.ether_type()
    }

    pub fn set_ether_type(&mut self, _ether_type: u16) -> &mut Self {
        self
    }

    pub fn new(
        arena: Arena,
        dst_addr: MacAddress,
        src_addr: MacAddress,
        ether_type: u16,
        data: &'a [u8],
    ) -> Self {
        Self {
            header: EthernetHeader::new(dst_addr, src_addr, ether_type, arena),
            data: data,
        }
    }
}

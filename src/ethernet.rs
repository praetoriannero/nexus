use crate::pdu::Pdu;
use crate::utils::parse_bytes;

static ETH_DST_OFFSET: usize = 0;
static ETH_SRC_OFFSET: usize = 6;
static ETH_TYPE_OFFSET: usize = 12;
static ETH_HEADER_LEN: usize = 14;

#[derive(Debug)]
struct EthernetHeader {
    dst_addr: [u8; 6],
    src_addr: [u8; 6],
    ether_type: u16,
}

#[derive(Debug)]
pub struct Ethernet<'a> {
    bytes: Option<&'a [u8]>,
    header: Option<EthernetHeader>,
    pub pdu_chain: Option<Vec<&'a Pdu<'a>>>,
    pub data: Option<&'a [u8]>,
    pub child_pdu: Option<&'a Pdu<'a>>,
    pub parent_pdu: Option<&'a Pdu<'a>>,
}

impl<'a> Ethernet<'a> {
    pub fn from_bytes(bytes: &'a [u8], pdu_chain: Option<Vec<&'a Pdu>>) -> Option<Self> {
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

    pub fn new(dst_addr: &[u8; 6], src_addr: &[u8; 6], ether_type: u16) -> Self {
        Self {
            bytes: None,
            parent_pdu: None,
            child_pdu: None,
            pdu_chain: None,
            data: None,
            header: Some(EthernetHeader {
                dst_addr: *dst_addr,
                src_addr: *src_addr,
                ether_type: ether_type,
            }),
        }
    }
}

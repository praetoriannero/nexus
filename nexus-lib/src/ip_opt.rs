use crate::error::ParseError;
use crate::pdu::{Pdu, Pob};

use nexus_macros::{Tid, pdu_impl, pdu_type};
use nexus_tid::Tid;
use std::any::TypeId;
use std::borrow::Cow;

const IPV4_OPT_TYPE_OFFSET: usize = 0;
const IPV4_OPT_SIZE_OFFSET: usize = 1;
const IPV4_OPT_DATA_OFFSET: usize = 3;
const IPV4_OPT_SIZE: usize = 2;

pub fn get_ip_opt_type(bytes: &[u8]) -> u8 {
    bytes[IPV4_OPT_TYPE_OFFSET]
}

pub const END: u8 = 0;
pub const NOP: u8 = 1;
pub const SEC: u8 = 2;
pub const LSR: u8 = 3;
pub const SSR: u8 = 9;
pub const REC: u8 = 7;
pub const SID: u8 = 8;
pub const ITS: u8 = 4;

/// Refer to https://datatracker.ietf.org/doc/html/rfc791#section-3.1
pub fn get_ip_opt_length(bytes: &[u8]) -> Result<usize, ParseError> {
    let opt_len = match get_ip_opt_type(bytes) {
        END => 1,
        NOP => 1,
        SEC => 11,
        SID => 4,
        LSR | ITS | SSR | REC => IPV4_OPT_SIZE + bytes[IPV4_OPT_SIZE_OFFSET] as usize,
        _ => 0,
    };
    if opt_len > 0 {
        Ok(opt_len)
    } else {
        Err(ParseError::UnsupportedProtocol)
    }
}

#[pdu_type]
pub struct IpOption<'a> {}

#[pdu_impl]
impl<'a> Pdu<'a> for IpOption<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.data);
        res
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError> {
        Ok(Self {
            header: Cow::Borrowed(&bytes),
            data: Cow::Owned(Vec::new()),
            parent: None,
            child: None,
        })
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        todo!()
    }
}

impl<'a> IpOption<'a> {
    pub fn new() -> Self {
        Self {
            header: Cow::Owned(vec![0; IPV4_OPT_SIZE]),
            data: Cow::Owned(Vec::new()),
            child: None,
            parent: None,
        }
    }

    pub fn opt_type(&self) -> u8 {
        self.header[IPV4_OPT_TYPE_OFFSET]
    }

    pub fn set_opt_type(&mut self, opt_type: u8) {
        self.header.to_mut()[IPV4_OPT_TYPE_OFFSET] = opt_type;
    }

    pub fn with_opt_type(&mut self, opt_type: u8) -> &mut Self {
        self.set_opt_type(opt_type);
        self
    }

    pub fn opt_length(&self) -> u8 {
        self.header[IPV4_OPT_SIZE_OFFSET]
    }

    pub fn set_opt_length(&mut self, opt_type: u8) {
        self.header.to_mut()[IPV4_OPT_SIZE_OFFSET] = opt_type;
    }

    pub fn with_opt_length(&mut self, opt_type: u8) -> &mut Self {
        self.set_opt_type(opt_type);
        self
    }

    pub fn opt_data(&self) -> &[u8] {
        &self.header[IPV4_OPT_DATA_OFFSET..self.opt_length() as usize]
    }

    pub fn set_opt_data(&mut self, data: &[u8]) {
        self.data.to_mut().extend_from_slice(data);
    }

    pub fn with_opt_data(&mut self, data: &[u8]) -> &mut Self {
        self.set_opt_data(data);
        self
    }
}

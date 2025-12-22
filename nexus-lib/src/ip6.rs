use crate::ethernet::{ETHER_DISSECTION_TABLE, EtherType};
use crate::prelude::*;

const IPV6_HEADER_LEN: usize = 40;

#[pdu_type]
pub struct Ipv6<'a> {}

#[pdu_impl]
impl<'a> Pdu<'a> for Ipv6<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.header);
        res
    }

    default_pdu_clone!(Ipv6);

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        Ok(Box::new(Self {
            header: Cow::Borrowed(&bytes[..IPV6_HEADER_LEN]),
            parent: None,
            child: None,
        }))
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        Ok(json!({
            "ipv6": {
                "ipv6.data": self.child_to_json(),
            }
        }))
    }
}

register_pdu!(EtherType(0x86DD), Ipv6, ETHER_DISSECTION_TABLE);

#[derive(Hash, Eq, PartialEq)]
pub struct Ipv6Type(pub u8);

pub static IPV6_DISSECTION_TABLE: DissectionTable<Ipv6Type> = create_table();

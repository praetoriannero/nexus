use crate::error::ParseError;
use crate::pdu::{Pdu, Pob};
use crate::utils::printable_ascii;

use nexus_macros::{Tid, pdu_impl, pdu_type};
use nexus_tid::Tid;
use serde_json::json;
use std::any::TypeId;
use std::borrow::Cow;

#[pdu_type]
pub struct Raw<'a> {}

#[pdu_impl]
impl<'a> Pdu<'a> for Raw<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.data);
        res
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        Ok(Box::new(Self {
            data: Cow::Borrowed(&bytes),
            header: Cow::Owned(Vec::new()),
            parent: None,
            child: None,
        }))
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        Ok(json!({
            "data": printable_ascii(&self.data)
        }))
    }
}

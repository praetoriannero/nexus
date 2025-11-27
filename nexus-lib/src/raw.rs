use std::any::TypeId;
use std::borrow::Cow;

use crate::error::ParseError;
use crate::pdu::{Pdu, Pob};
use nexus_macros::{Tid, pdu_impl, pdu_type};
use nexus_tid::Tid;

#[pdu_type]
pub struct Raw<'a> {}

#[pdu_impl]
impl<'a> Pdu<'a> for Raw<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.data);
        res
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError> {
        Ok(Self {
            data: Cow::Borrowed(&bytes),
            header: Cow::Owned(Vec::new()),
            parent: None,
            child: None,
        })
    }
}

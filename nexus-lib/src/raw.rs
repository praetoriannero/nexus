use std::any::TypeId;
use std::borrow::Cow;

use crate::error::ParseError;
use crate::pdu::{Pdu, Pob};
use nexus_macros::Tid;
use nexus_tid::Tid;

#[derive(Tid)]
pub struct Raw<'a> {
    data: Cow<'a, [u8]>,
    parent: Pob<'a>,
    child: Pob<'a>,
}

impl<'a> Pdu<'a> for Raw<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.data);
        res
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError> {
        Ok(Self {
            data: Cow::Borrowed(&bytes),
            parent: None,
            child: None,
        })
    }

    fn parent_pdu(&self) -> &Pob<'a> {
        &self.parent
    }

    fn child_pdu(&self) -> &Pob<'a> {
        &self.child
    }
}

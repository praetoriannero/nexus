use crate::error::ParseError;

// use better_any::{Tid, TidAble};

pub trait Pdu<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError>
    where
        Self: Sized;

    fn to_bytes(&self) -> Result<Vec<u8>, ParseError>
    where
        Self: Sized;

    fn pdu_type(&self) -> PduType;

    // fn as_any(&self) -> &dyn Tid<'a>
    // where
    //     Self: TidAble<'a> + Sized,
    // {
    //     self
    // }
}

pub enum PduType {
    Ethernet,
    Ip,
}

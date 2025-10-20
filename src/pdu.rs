use crate::error::ParseError;

pub trait Pdu<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError>
    where
        Self: Sized;
    fn to_bytes(&self) -> Result<Vec<u8>, ParseError>
    where
        Self: Sized;
    // fn prepare(&'a mut self);
    // fn finalize(&'a mut self);
}

use crate::ip::Ip;

#[derive(Debug)]
pub enum Pdu<'a> {
    Ip(Ip<'a>),
}

pub trait Deserialize<'a> {
    fn from_bytes(bytes: &'a [u8], pdu_chain: Option<Vec<&'a Pdu>>) -> Option<Self>
    where
        Self: Sized;
}

pub trait Serialize<'a> {
    fn finalize(&'a mut self);
}

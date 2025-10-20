use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    NotEnoughData,
    InvalidHeader,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pdu parsing failed")
    }
}

impl Error for ParseError {}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    NotEnoughData,
    InvalidHeader,
    UnsupportedProtocol,
}

#[derive(Debug)]
pub enum AllocError {
    InsufficientSpace,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pdu parsing failed")
    }
}

impl Error for ParseError {}

impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "insufficient space")
    }
}

impl Error for AllocError {}

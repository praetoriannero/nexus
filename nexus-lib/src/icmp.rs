use crate::prelude::*;

#[pdu_type]
pub struct EchoReply<'a> {}

impl<'a> EchoReply<'a> {
    pub fn new() -> Self {
        Self {
            header: Cow::Owned(Vec::new()),
            data: Cow::Owned(Vec::new()),
            parent: None,
            child: None,
        }
    }
}

#[pdu_impl]
impl<'a> Pdu<'a> for EchoReply<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError>
    where
        Self: Sized,
    {
        Ok(Box::new(Self {
            header: Cow::Owned(Vec::new()),
            data: Cow::Borrowed(&bytes),
            parent: None,
            child: None,
        }))
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::error::Error> {
        todo!()
    }
}

#[pdu_type]
pub struct DestUnreachable<'a> {}

#[pdu_type]
pub struct RedirectMessage<'a> {}

#[pdu_type]
pub struct EchoRequest<'a> {}

#[pdu_type]
pub struct RouterAdvertisement<'a> {}

#[pdu_type]
pub struct RouterSolicitation<'a> {}

#[pdu_type]
pub struct TimeExceeded<'a> {}

#[pdu_type]
pub struct ParameterProblem<'a> {}

#[pdu_type]
pub struct Timestamp<'a> {}

#[pdu_type]
pub struct TimestampReply<'a> {}

#[pdu_type]
pub struct InfoRequest<'a> {}

#[pdu_type]
pub struct InfoReply<'a> {}

#[pdu_type]
pub struct AddrMaskRequest<'a> {}

#[pdu_type]
pub struct AddrMaskReply<'a> {}

#[pdu_type]
pub struct Traceroute<'a> {}

#[pdu_type]
pub struct ExtEchoRequest<'a> {}

#[pdu_type]
pub struct ExtEchoReply<'a> {}

pub enum ControlMessage<'a> {
    EchoReply(EchoReply<'a>),
    EchoRequest(EchoRequest<'a>),
    DestUnreachable(DestUnreachable<'a>),
    RedirectMessage(RedirectMessage<'a>),
    RouterAdvertisement(RouterAdvertisement<'a>),
    RouterSolicitation(RouterSolicitation<'a>),
    TimeExceeded(TimeExceeded<'a>),
    ParameterProblem(ParameterProblem<'a>),
    Timestamp(Timestamp<'a>),
    TimestampReply(TimestampReply<'a>),
    InfoRequest(InfoRequest<'a>),
    InfoReply(InfoReply<'a>),
    AddrMaskRequest(AddrMaskRequest<'a>),
    AddrMaskReply(AddrMaskReply<'a>),
    Traceroute(Traceroute<'a>),
    ExtEchoRequest(ExtEchoRequest<'a>),
    ExtEchoReply(ExtEchoReply<'a>),
}

const ICMP_MIN_SIZE: usize = 4;

#[pdu_type]
pub struct Icmp<'a> {
    msg: ControlMessage<'a>,
}

#[pdu_impl]
impl<'a> Pdu<'a> for Icmp<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.header);
        res.extend_from_slice(&self.data);
        res
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        Ok(Box::new(Self {
            header: Cow::Borrowed(&bytes[..ICMP_MIN_SIZE]),
            data: Cow::Borrowed(&bytes[ICMP_MIN_SIZE..]),
            msg: ControlMessage::EchoReply(EchoReply::new()),
            parent: None,
            child: None,
        }))
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        Ok(json!({
            "ipv6": {
                "ipv6.data": printable_ascii(&self.data)
            }
        }))
    }
}

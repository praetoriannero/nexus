use crate::{prelude::*, register_pdu};

const ECHO_REPLY_ID_OFFSET: usize = 0;
const ECHO_REPLY_SEQNUM_OFFSET: usize = 2;
const ECHO_REPLY_DATA_OFFSET: usize = 4;

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

    pub fn id(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[ECHO_REPLY_ID_OFFSET..ECHO_REPLY_SEQNUM_OFFSET],
            Endian::Big,
        )
    }

    pub fn set_id(&mut self, id: u16) {
        self.header.to_mut()[ECHO_REPLY_ID_OFFSET..ECHO_REPLY_SEQNUM_OFFSET]
            .copy_from_slice(&id.to_be_bytes());
    }

    pub fn with_id(&mut self, id: u16) -> &mut Self {
        self.set_id(id);
        self
    }

    pub fn seq_number(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[ECHO_REPLY_SEQNUM_OFFSET..ECHO_REPLY_DATA_OFFSET],
            Endian::Big,
        )
    }

    pub fn set_seq_number(&mut self, seq_number: u16) {
        self.header.to_mut()[ECHO_REPLY_SEQNUM_OFFSET..ECHO_REPLY_DATA_OFFSET]
            .copy_from_slice(&seq_number.to_be_bytes());
    }

    pub fn with_seq_number(&mut self, seq_number: u16) -> &mut Self {
        self.set_seq_number(seq_number);
        self
    }

    pub fn data(&self) -> &[u8] {
        &self.header[ECHO_REPLY_DATA_OFFSET..]
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.data = Cow::Owned(data.to_vec());
    }

    pub fn with_data(&mut self, data: &[u8]) -> &mut Self {
        self.set_data(data);
        self
    }
}

#[pdu_impl]
impl<'a> Pdu<'a> for EchoReply<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        Ok(Box::new(Self {
            header: Cow::Borrowed(&bytes[..ECHO_REPLY_DATA_OFFSET]),
            data: Cow::Borrowed(&bytes[ECHO_REPLY_DATA_OFFSET..]),
            parent: None,
            child: None,
        }))
    }

    default_to_owned!(EchoReply);

    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.header);
        res.extend_from_slice(&self.data);
        res
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::error::Error> {
        Ok(json!({
            "echo_reply": {
                "echo_reply.id": self.id(),
                "echo_reply.seq_number": self.seq_number(),
                "echo_reply.data": printable_ascii(&self.data)
            }
        }))
    }
}

register_pdu!(IcmpType(0), EchoReply, ICMP_DISSECTION_TABLE);

#[pdu_type]
pub struct DestUnreachable<'a> {}

// impl<'a> DestUnreachable<'a> {
//     pub fn new() {
//         Self {
//             header: Cow::Owned(),
//         }
//     }
// }

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
pub struct ExtEchoRequest<'a> {}

#[pdu_type]
pub struct ExtEchoReply<'a> {}

#[derive(Hash, Eq, PartialEq)]
pub struct IcmpType(pub u8);

pub static ICMP_DISSECTION_TABLE: DissectionTable<IcmpType> = create_table();

// register_pdu!(IcmpType(3), DestUnreachable, ICMP_DISSECTION_TABLE);
// register_pdu!(IcmpType(5), RedirectMessage, ICMP_DISSECTION_TABLE);

#[derive(Clone)]
pub enum ControlMessage {
    EchoReply = 0,
    EchoRequest = 8,
    DestUnreachable = 3,
    RedirectMessage = 5,
    RouterAdvertisement = 9,
    RouterSolicitation = 10,
    TimeExceeded = 11,
    ParameterProblem = 12,
    Timestamp = 13,
    TimestampReply = 14,
    ExtEchoRequest = 42,
    ExtEchoReply = 43,
    Undefined,
}

const ICMP_MIN_SIZE: usize = 4;

#[pdu_type]
pub struct Icmp<'a> {
    pub msg_type: ControlMessage,
    pub msg_body: Pob<'a>,
    header_len: usize,
}

impl<'a> Icmp<'a> {
    pub fn msg_type(&self) -> u8 {
        0
    }

    pub fn msg_code(&self) -> u8 {
        0
    }
}

#[pdu_impl]
impl<'a> Pdu<'a> for Icmp<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.header);
        res.extend_from_slice(&self.data);
        res
    }

    fn to_owned(&self) -> Box<dyn Pdu<'static>> {
        Box::new(Icmp {
            header: Cow::Owned(self.header.to_vec()),
            data: Cow::Owned(self.data.to_vec()),
            child: None,
            parent: None,
            msg_type: self.msg_type.clone(),
            msg_body: None,
            header_len: self.header_len,
        })
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        // TODO: parse message type
        Ok(Box::new(Self {
            header: Cow::Borrowed(&bytes[..ICMP_MIN_SIZE]),
            data: Cow::Borrowed(&bytes[ICMP_MIN_SIZE..]),
            msg_type: ControlMessage::Undefined,
            msg_body: None,
            header_len: ICMP_MIN_SIZE,
            parent: None,
            child: None,
        }))
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        Ok(json!({
            "icmp": {
                "icmp.data": printable_ascii(&self.data)
            }
        }))
    }
}

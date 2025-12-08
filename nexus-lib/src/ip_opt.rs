use crate::error::ParseError;
use crate::pdu::{Pdu, Pob};

use nexus_macros::{Tid, pdu_impl, pdu_type};
use nexus_tid::Tid;
use std::any::TypeId;
use std::borrow::Cow;

const IPV4_OPT_TYPE_OFFSET: usize = 0;
const IPV4_OPT_SIZE_OFFSET: usize = 1;
const IPV4_OPT_DATA_OFFSET: usize = 3;
const IPV4_OPT_SIZE: usize = 2;

pub fn get_ip_opt_type(bytes: &[u8]) -> u8 {
    bytes[IPV4_OPT_TYPE_OFFSET]
}

pub const END: u8 = 0;
pub const NOP: u8 = 1;
pub const SEC: u8 = 2;
pub const LSR: u8 = 3;
pub const SSR: u8 = 9;
pub const REC: u8 = 7;
pub const SID: u8 = 8;
pub const ITS: u8 = 4;

/// Refer to <https://datatracker.ietf.org/doc/html/rfc791#section-3.1>
pub fn get_ip_opt_length(bytes: &[u8]) -> Result<usize, ParseError> {
    let opt_len = match get_ip_opt_type(bytes) {
        END => 1,
        NOP => 1,
        SEC => 11,
        SID => 4,
        LSR | ITS | SSR | REC => IPV4_OPT_SIZE + bytes[IPV4_OPT_SIZE_OFFSET] as usize,
        _ => 0,
    };
    if opt_len > 0 {
        Ok(opt_len)
    } else {
        Err(ParseError::UnsupportedProtocol)
    }
}

#[pdu_type]
pub struct IpOption<'a> {}

#[pdu_impl]
impl<'a> Pdu<'a> for IpOption<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.data);
        res
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        Ok(Box::new(Self {
            header: Cow::Borrowed(&bytes),
            data: Cow::Owned(Vec::new()),
            parent: None,
            child: None,
        }))
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        todo!()
    }
}

impl<'a> IpOption<'a> {
    pub fn new() -> Self {
        Self {
            header: Cow::Owned(vec![0; IPV4_OPT_SIZE]),
            data: Cow::Owned(Vec::new()),
            child: None,
            parent: None,
        }
    }

    pub fn opt_type(&self) -> u8 {
        self.header[IPV4_OPT_TYPE_OFFSET]
    }

    pub fn set_opt_type(&mut self, opt_type: u8) {
        self.header.to_mut()[IPV4_OPT_TYPE_OFFSET] = opt_type;
    }

    pub fn with_opt_type(&mut self, opt_type: u8) -> &mut Self {
        self.set_opt_type(opt_type);
        self
    }

    pub fn opt_length(&self) -> u8 {
        self.header[IPV4_OPT_SIZE_OFFSET]
    }

    pub fn set_opt_length(&mut self, opt_type: u8) {
        self.header.to_mut()[IPV4_OPT_SIZE_OFFSET] = opt_type;
    }

    pub fn with_opt_length(&mut self, opt_type: u8) -> &mut Self {
        self.set_opt_type(opt_type);
        self
    }

    pub fn opt_data(&self) -> &[u8] {
        &self.header[IPV4_OPT_DATA_OFFSET..self.opt_length() as usize]
    }

    pub fn set_opt_data(&mut self, data: &[u8]) {
        self.data.to_mut().extend_from_slice(data);
    }

    pub fn with_opt_data(&mut self, data: &[u8]) -> &mut Self {
        self.set_opt_data(data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ip::Ip;

    const IPV4_NO_OPTIONS: &[u8] = &[
        0x45, 0x00, // Version=4, IHL=5, DSCP/ECN
        0x00, 0x1c, // Total Length = 28 bytes
        0x12, 0x34, // Identification
        0x40, 0x00, // Flags=DF, Fragment Offset
        0x40, // TTL=64
        0x06, // Protocol=TCP
        0xb9, 0xe6, // Header checksum
        0xc0, 0xa8, 0x00, 0x01, // Src = 192.168.0.1
        0xc0, 0xa8, 0x00, 0x02, // Dst = 192.168.0.2
        0xde, 0xad, 0xbe, 0xef, // Payload (4 bytes)
    ];

    const IPV4_RR: &[u8] = &[
        0x47, 0x00, // Version=4, IHL=7 (28 bytes header)
        0x00, 0x24, // Total Length = 36 bytes
        0xab, 0xcd, // Identification
        0x00, 0x00, // Flags/Fragment offset
        0x40, // TTL=64
        0x11, // Protocol=UDP
        0x67, 0x2b, // Header checksum
        0x0a, 0x00, 0x00, 0x01, // Src = 10.0.0.1
        0x0a, 0x00, 0x00, 0x02, // Dst = 10.0.0.2
        // ---- IPv4 OPTIONS (8 bytes) ----
        0x07, // RR (type 7)
        0x07, // Length = 7 bytes
        0x04, // Pointer = 4 (next entry location)
        0x00, 0x00, 0x00, 0x00, // Space for 1 route address (unused)
        0x00, // End of Options List (EOL)
        // Payload
        0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x01, 0x02,
    ];

    const IPV4_MIXED_OPTIONS: &[u8] = &[
        0x48, 0x00, // Version=4, IHL=8
        0x00, 0x28, // Total Length = 40 bytes
        0xde, 0xad, 0x00, 0x00, 0x40, 0x01, // TTL, Protocol=ICMP
        0x15, 0x7a, // Checksum
        0x7f, 0x00, 0x00, 0x01, // Src 127.0.0.1
        0x7f, 0x00, 0x00, 0x01, // Dst 127.0.0.1
        // ---- Options (12 bytes) ----
        0x01, // NOP
        0x01, // NOP again
        0x07, 0x07, 0x04, // RR (type=7, len=7, pointer=4)
        0x00, 0x00, 0x00, 0x00, // 1 RR entry (empty)
        0x00, // EOL
        0x00, 0x00, 0x00, // Padding to 32-byte header
        // Payload (8 bytes)
        0xaa, 0xaa, 0xaa, 0xaa, 0xbb, 0xbb, 0xbb, 0xbb,
    ];

    const IPV4_TS: &[u8] = &[
        0x48, 0x00, // Version=4, IHL=8
        0x00, 0x28, // Total Length
        0x55, 0x55, 0x00, 0x00, 0x40, 0x11, // TTL=64, UDP
        0x3a, 0x79, // Header checksum
        0xc0, 0xa8, 0x00, 0x64, // Src 192.168.0.100
        0xc0, 0xa8, 0x00, 0x65, // Dst 192.168.0.101
        // ---- Timestamp option (10 bytes) ----
        0x44, // Type = Timestamp
        0x0a, // Length = 10
        0x05, // Pointer = 5 (next timestamp slot)
        0x00, // Overflow + flags = 0
        0x00, 0x00, 0x00, 0x01, // Timestamp entry (1)
        0x00, 0x00, // Padding to IHL
        // Payload
        0xde, 0xad, 0xfa, 0xce, 0x12, 0x34, 0x56, 0x78,
    ];

    const IPV4_SECURITY: &[u8] = &[
        0x48, 0x00, // Version=4, IHL=8
        0x00, 0x28, // Total length
        0x01, 0x23, 0x00, 0x00, 0x40, 0x06, // TTL=64, TCP
        0x6d, 0xbd, // Header checksum
        0xac, 0x10, 0x00, 0x01, // Src 172.16.0.1
        0xac, 0x10, 0x00, 0x02, // Dst 172.16.0.2
        // ---- Security Option (11 bytes) ----
        0x82, // Type = Security
        0x0b, // Length = 11 bytes
        0xf1, 0xf0, // Security
        0x00, 0x00, // Compartment
        0x00, 0x00, // Handling restrictions
        0x00, 0x01, // TCC
        0x00, // Padding to 32 bytes
        // Payload (8 bytes)
        0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x11, 0x22,
    ];

    #[test]
    fn test_no_opt() {}

    #[test]
    fn test_rr_opt() {}

    #[test]
    fn test_mixed_opt() {}

    #[test]
    fn test_ts_opt() {}

    #[test]
    fn test_sec_opt() {}
}

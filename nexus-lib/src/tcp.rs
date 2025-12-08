use crate::ip::{IPV4_DISSECTION_TABLE, Ipv4Type};
use crate::prelude::*;
use crate::register_ipv4_type;

const TCP_MIN_HEADER_LEN: usize = 24;
const TCP_DATA_SIZE_OFFSET: usize = 12;
const TCP_HEADER_MULT: usize = 4;
const TCP_SPORT_OFFSET: usize = 0;
const TCP_DPORT_OFFSET: usize = 2;
const TCP_SQNUM_OFFSET: usize = 4;
const TCP_AKNUM_OFFSET: usize = 8;
const TCP_DR_OFFSET: usize = 12;
const TCP_FLAGS_OFFSET: usize = 13;
const TCP_WINDOW_OFFSET: usize = 14;
const TCP_CHECKSUM_OFFSET: usize = 16;
const TCP_URGPTR_OFFSET: usize = 18;

#[pdu_type]
pub struct Tcp<'a> {}

fn get_data_offset<'a>(bytes: &'a [u8]) -> usize {
    (bytes[TCP_DATA_SIZE_OFFSET] >> 4) as usize * TCP_HEADER_MULT
}

#[pdu_impl]
impl<'a> Pdu<'a> for Tcp<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.data);
        res
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        let header_size = get_data_offset(bytes);
        println!("tcp deserialize attempt {} {}", header_size, bytes.len());
        if header_size > bytes.len() {
            return Err(ParseError::NotEnoughData);
        }

        Ok(Box::new(Self {
            header: Cow::Borrowed(&bytes[..header_size]),
            data: Cow::Borrowed(&bytes[header_size..]),
            parent: None,
            child: None,
        }))
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        Ok(json!({
            "tcp": {
                "tcp.sport": self.src_port(),
                "tcp.dport": self.dst_port(),
                "tcp.seq_number": self.seq_number(),
                "tcp.ack_number": self.ack_number(),
                "tcp.data_offset": self.data_offset(),
                "tcp.reserved": self.reserved(),
                "tcp.flags": self.flags(),
                "tcp.window": self.window(),
                "tcp.checksum": self.checksum(),
                "tcp.urg_pointer": self.urg_pointer(),
                "tcp.data": printable_ascii(&self.data),
            }
        }))
    }
}

impl<'a> Tcp<'a> {
    pub fn src_port(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[TCP_SPORT_OFFSET..TCP_DPORT_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_src_port(&mut self, sport: u16) {
        self.header.to_mut()[TCP_SPORT_OFFSET..TCP_DPORT_OFFSET]
            .copy_from_slice(&sport.to_be_bytes());
    }

    pub fn with_src_port(&mut self, sport: u16) -> &mut Self {
        self.set_src_port(sport);
        self
    }

    pub fn dst_port(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[TCP_DPORT_OFFSET..TCP_SQNUM_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_dst_port(&mut self, dport: u16) {
        self.header.to_mut()[TCP_DPORT_OFFSET..TCP_SQNUM_OFFSET]
            .copy_from_slice(&dport.to_be_bytes());
    }

    pub fn with_dst_port(&mut self, dport: u16) -> &mut Self {
        self.set_dst_port(dport);
        self
    }

    pub fn seq_number(&self) -> u32 {
        parse_bytes::<u32>(
            &self.header[TCP_SQNUM_OFFSET..TCP_AKNUM_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_seq_number(&mut self, seq_num: u32) {
        self.header.to_mut()[TCP_SQNUM_OFFSET..TCP_AKNUM_OFFSET]
            .copy_from_slice(&seq_num.to_be_bytes());
    }

    pub fn with_seq_number(&mut self, seq_num: u32) -> &mut Self {
        self.set_seq_number(seq_num);
        self
    }

    pub fn ack_number(&self) -> u32 {
        parse_bytes::<u32>(
            &self.header[TCP_AKNUM_OFFSET..TCP_DR_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_ack_number(&mut self, ack_num: u32) {
        self.header.to_mut()[TCP_AKNUM_OFFSET..TCP_AKNUM_OFFSET]
            .copy_from_slice(&ack_num.to_be_bytes());
    }

    pub fn with_ack_number(&mut self, ack_num: u32) -> &mut Self {
        self.set_ack_number(ack_num);
        self
    }

    pub fn data_offset(&self) -> u8 {
        (self.header[TCP_DR_OFFSET] >> 4) * 4
    }

    pub fn set_data_offset(&mut self, data_offset: u8) {
        let data_offset_bits = &mut self.header.to_mut()[TCP_DR_OFFSET];
        let mask = 0b0000_1111;
        *data_offset_bits &= mask;
        *data_offset_bits &= data_offset << 4;
    }

    pub fn with_data_offset(&mut self, data_offset: u8) -> &mut Self {
        self.set_data_offset(data_offset);
        self
    }

    pub fn reserved(&self) -> u8 {
        self.header[TCP_DR_OFFSET] & 0b0000_1111
    }

    pub fn set_reserved(&mut self, reserved: u8) {
        let reserved_bits = &mut self.header.to_mut()[TCP_DR_OFFSET];
        let mask = 0b1111_0000;
        *reserved_bits &= mask;
        *reserved_bits &= reserved;
    }

    pub fn with_reserved(&mut self, reserved: u8) -> &mut Self {
        self.set_reserved(reserved);
        self
    }

    pub fn flags(&self) -> u8 {
        self.header[TCP_FLAGS_OFFSET]
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.header.to_mut()[TCP_FLAGS_OFFSET] = flags;
    }

    pub fn with_flags(&mut self, flags: u8) -> &mut Self {
        self.set_flags(flags);
        self
    }

    pub fn window(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[TCP_WINDOW_OFFSET..TCP_CHECKSUM_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_window(&mut self, window: u16) {
        self.header.to_mut()[TCP_WINDOW_OFFSET..TCP_CHECKSUM_OFFSET]
            .copy_from_slice(&window.to_be_bytes());
    }

    pub fn with_window(&mut self, window: u16) -> &mut Self {
        self.set_window(window);
        self
    }

    pub fn checksum(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[TCP_CHECKSUM_OFFSET..TCP_URGPTR_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_checksum(&mut self, checksum: u16) {
        self.header.to_mut()[TCP_CHECKSUM_OFFSET..TCP_AKNUM_OFFSET]
            .copy_from_slice(&checksum.to_be_bytes());
    }

    pub fn with_checksum(&mut self, checksum: u16) -> &mut Self {
        self.set_checksum(checksum);
        self
    }

    pub fn urg_pointer(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[TCP_URGPTR_OFFSET..TCP_MIN_HEADER_LEN],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_urg_pointer(&mut self, urg_pointer: u16) {
        self.header.to_mut()[TCP_URGPTR_OFFSET..TCP_MIN_HEADER_LEN]
            .copy_from_slice(&urg_pointer.to_be_bytes());
    }
}

register_ipv4_type!(Ipv4Type(0x6), Tcp);

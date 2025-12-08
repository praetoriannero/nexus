use crate::ip::{IPV4_DISSECTION_TABLE, Ipv4Type};
use crate::prelude::*;
use crate::register_ipv4_type;

pub const UDP_HEADER_LEN: usize = 32;
const UDP_SPORT_OFFSET: usize = 0;
const UDP_DPORT_OFFSET: usize = 2;
const UDP_LENGTH_OFFSET: usize = 4;
const UDP_CHECKSUM_OFFSET: usize = 6;

#[pdu_type]
pub struct Udp<'a> {}

#[pdu_impl]
impl<'a> Pdu<'a> for Udp<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.header);
        res.extend_from_slice(&self.data);
        res
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        Ok(Box::new(Self {
            data: Cow::Borrowed(&bytes[UDP_HEADER_LEN..]),
            header: Cow::Borrowed(&bytes[..UDP_HEADER_LEN]),
            parent: None,
            child: None,
        }))
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        Ok(json!({
            "data": printable_ascii(&self.data)
        }))
    }
}

impl<'a> Udp<'a> {
    pub fn src_port(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[UDP_SPORT_OFFSET..UDP_DPORT_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_src_port(&mut self, sport: u16) {
        self.header.to_mut()[UDP_SPORT_OFFSET..UDP_DPORT_OFFSET]
            .copy_from_slice(&sport.to_be_bytes());
    }

    pub fn with_src_port(&mut self, sport: u16) -> &mut Self {
        self.set_src_port(sport);
        self
    }

    pub fn dst_port(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[UDP_DPORT_OFFSET..UDP_LENGTH_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_dst_port(&mut self, dport: u16) {
        self.header.to_mut()[UDP_DPORT_OFFSET..UDP_LENGTH_OFFSET]
            .copy_from_slice(&dport.to_be_bytes());
    }

    pub fn with_dst_port(&mut self, dport: u16) -> &mut Self {
        self.set_dst_port(dport);
        self
    }

    pub fn length(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[UDP_LENGTH_OFFSET..UDP_CHECKSUM_OFFSET],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_length(&mut self, length: u16) {
        self.header.to_mut()[UDP_LENGTH_OFFSET..UDP_CHECKSUM_OFFSET]
            .copy_from_slice(&length.to_be_bytes());
    }

    pub fn with_length(&mut self, length: u16) -> &mut Self {
        self.set_length(length);
        self
    }

    pub fn checksum(&self) -> u16 {
        parse_bytes::<u16>(
            &self.header[UDP_CHECKSUM_OFFSET..UDP_HEADER_LEN],
            crate::utils::Endian::Big,
        )
    }

    pub fn set_checksum(&mut self, checksum: u16) {
        self.header.to_mut()[UDP_CHECKSUM_OFFSET..UDP_HEADER_LEN]
            .copy_from_slice(&checksum.to_be_bytes());
    }

    pub fn with_checksum(&mut self, checksum: u16) -> &mut Self {
        self.set_checksum(checksum);
        self
    }
}

register_ipv4_type!(Ipv4Type(0x11), Udp);

pub struct UdpType(pub u16);

pub static UDP_DISSECTION_TABLE: LazyLock<RwLock<HashMap<UdpType, PduBuilder>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[macro_export]
macro_rules! register_udp_type {
    ($udp_type:expr, $builder:ident) => {
        paste! {
            #[ctor]
            fn [<__nexus_register_udp_type_ $builder:lower>]() {
                pdu_trait_assert::<$builder>();
                if UDP_DISSECTION_TABLE
                    .write()
                    .unwrap()
                    .insert($udp_type, |bytes: &'_ [u8]| -> PduResult<'_> {
                        $builder::from_bytes(bytes)
                    })
                    .is_some()
                {
                    panic!("UDP types can only be added once.")
                };
            }
        }
    };
}

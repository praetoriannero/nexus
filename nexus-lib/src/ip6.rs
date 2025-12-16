use crate::ethernet::{ETHER_DISSECTION_TABLE, EtherType};
use crate::{prelude::*, register_pdu};

const IPV6_HEADER_LEN: usize = 40;

#[pdu_type]
pub struct Ipv6<'a> {}

#[pdu_impl]
impl<'a> Pdu<'a> for Ipv6<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.header);
        res.extend_from_slice(&self.data);
        res
    }

    default_to_owned!(Ipv6);

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        Ok(Box::new(Self {
            data: Cow::Borrowed(&bytes[IPV6_HEADER_LEN..]),
            header: Cow::Borrowed(&bytes[..IPV6_HEADER_LEN]),
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

register_pdu!(EtherType(0x86DD), Ipv6, ETHER_DISSECTION_TABLE);

#[derive(Hash, Eq, PartialEq)]
pub struct Ipv6Type(pub u8);

pub static IPV6_DISSECTION_TABLE: LazyLock<RwLock<HashMap<Ipv6Type, PduBuilder>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[macro_export]
macro_rules! register_ipv6_type {
    ($ipv6_type:expr, $builder:ident) => {
        paste! {
            #[ctor]
            fn [<__nexus_register_ipv6_type_ $builder:lower>]() {
                pdu_trait_assert::<$builder>();
                if IPV6_DISSECTION_TABLE
                    .write()
                    .unwrap()
                    .insert($ipv6_type, |bytes: &'_ [u8]| -> PduResult<'_> {
                        $builder::from_bytes(bytes)
                    })
                    .is_some()
                {
                    panic!("IPv6 types can only be registered once.")
                };
            }
        }
    };
}

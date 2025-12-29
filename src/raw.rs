use crate::default_pdu_clone;
use crate::prelude::*;

#[pdu_type]
pub struct Raw<'a> {}

#[pdu_impl]
impl<'a> Pdu<'a> for Raw<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.header);
        res
    }

    default_pdu_clone!(Raw);

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<dyn Pdu<'a> + 'a>, ParseError> {
        Ok(Box::new(Self {
            header: Cow::Borrowed(&bytes),
            parent: None,
            child: None,
        }))
    }

    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        Ok(json!({
            "raw": {
                "raw.data": printable_ascii(&self.header),
            }
        }))
    }
}

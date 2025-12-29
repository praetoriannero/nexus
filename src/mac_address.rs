use serde::Serialize;
use std::fmt::Display;

const MAC_ADDR_SIZE: usize = 6;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct MacAddress<'a> {
    address: &'a [u8; MAC_ADDR_SIZE],
}

impl<'a> MacAddress<'a> {
    pub fn into_buff(&self, buff: &mut [u8]) -> Result<(), std::array::TryFromSliceError> {
        self.address.clone_into(buff.try_into()?);
        Ok(())
    }

    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            address: bytes.try_into().expect("err"),
        }
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        self.address.clone()
    }

    pub fn to_str(&self) -> String {
        let str_nums: Vec<String> = self.address.iter().map(|n| format!("{:02X}", n)).collect();
        str_nums.join(":").clone()
    }
}

impl<'a> Display for MacAddress<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, byte) in self.address.iter().enumerate() {
            if i != 0 {
                write!(f, ":")?;
            }
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

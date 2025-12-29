use crate::bit_width::BitWidth;

pub trait BitSerde: BitWidth {
    fn to_bits(&self) -> Vec<u8>;
    fn from_bits(&self, bytes: &[u8]) -> Self
    where
        Self: Sized;
}

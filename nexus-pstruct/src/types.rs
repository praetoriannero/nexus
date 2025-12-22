#![allow(dead_code)]

// use num::ToPrimitive;
// use num_traits::
//
pub enum Alignment {
    Left,
    Right,
}

pub type ActivateCallback = fn(&[u8]) -> bool;

/// Constructs a byte array from a &[u8]
pub struct BytesField<const S: usize> {}

/// Used to skip bytes in a &[u8]
pub struct PadBytes<const S: usize> {}

/// Constructs a bits field
pub struct BitsField<const S: usize> {}

pub struct PadBits<const S: usize> {}

// pub struct Field<T: ToPrimitive> {}

pub struct Test {
    src: BytesField<4>,
    dst: BytesField<4>,
}

pub struct Field {
    pub byte_offset: usize,
    pub bit_offset: usize,
    pub size: usize,
    pub bit_field: bool,
    pub activate: Option<ActivateCallback>,
    pub repeated: bool,
    pub aligned: Alignment,
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let a = 100 > 0;
        assert!(a);
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}

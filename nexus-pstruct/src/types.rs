#![allow(dead_code)]

pub enum Alignment {
    Left,
    Right,
}

/// Activates a field
pub type ActivateCallback = fn(&[u8]) -> bool;

/// Determine number of repeats of a field
pub type RepeatCallback = fn(&[u8]) -> usize;

/// Constructs a byte array
pub struct BytesField<const S: usize, const O: usize> {}

/// Used to skip irrelevant bytes
pub struct PadBytes<const S: usize> {}

/// Constructs a bits field
pub struct BitsField<const S: usize> {}

/// Used to skip irrelevant bits
pub struct PadBits<const S: usize> {}

/// All Protocol fields must implement this
pub trait ProtoField {
    fn from_bytes(bytes: &[u8]);
    fn to_bytes(&self) -> Vec<u8>;
}

/// Contains metadata about the field
pub struct FieldMetadata {
    pub byte_offset: usize,
    pub bit_offset: usize,
    pub size: usize,
    pub bit_field: bool,
    pub activate: Option<ActivateCallback>,
    pub repeated: Option<RepeatCallback>,
    pub aligned: Alignment,
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Test {
        src: BytesField<4, 0>,
        dst: BytesField<4, 4>,
    }

    #[test]
    fn it_works() {
        let a = 100 > 0;
        assert!(a);
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}

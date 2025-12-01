use num_traits::Unsigned;

pub struct BytesField<const S: usize> {}

pub struct PadBytes<const S: usize> {}

pub struct BitsField<const S: usize> {}

pub struct PadBits<const S: usize> {}

pub struct Field<T: Unsigned> {}

pub struct Test {
    src: BytesField<4>,
    dst: BytesField<4>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = 100 as bool;
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}

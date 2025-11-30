use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, ItemImpl, parse_macro_input};

struct MyStruct {}

fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[proc_macro_derive(Protocol, attributes(bitfield, bytefield, pad, option))]
pub fn derive_protocol(input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as DeriveInput);
    let name = &input_struct.ident;

    let bitfields = match &input_struct.data {
        Data::Struct(s) => &s.fields,
        _ => return quote! {compile_error!("Pstruct only works on structs"); }.into(),
    };

    struct BitField {
        byte_position: u64,
        offset: u64,
        count: u64,
    }

    // let mut generated_methods = Vec::new();

    for field in bitfields {
        let ident = field.ident.as_ref().unwrap();

        let mut data = BitField {
            byte_position: 0,
            offset: 0,
            count: 0,
        };

        for attr in 
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

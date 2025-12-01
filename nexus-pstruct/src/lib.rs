use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, ItemImpl, parse_macro_input};

#[proc_macro_derive(Protocol, attributes(field))]
pub fn derive_protocol(input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as DeriveInput);
    let name = &input_struct.ident;

    let bitfields = match &input_struct.data {
        Data::Struct(s) => &s.fields,
        _ => return quote! {compile_error!("Pstruct only works on structs"); }.into(),
    };

    struct Field {
        byte_offset: usize,
        bit_offset: usize,
        size: usize,
        bit_field: bool,
        depends_on: Option<String>,
        repeated: bool,
    }

    // let mut generated_methods = Vec::new();

    for field in bitfields {
        let ident = field.ident.as_ref().unwrap();

        let mut data = Field {
            byte_offset: 0,
            bit_offset: 0,
            size: 0,
            bit_field: false,
            depends_on: None,
            repeated: false,
        };

        // for attr in
    }

    let expanded = quote! {#input_struct};
    expanded.into()
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

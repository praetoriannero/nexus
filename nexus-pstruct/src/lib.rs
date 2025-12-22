use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub(crate) mod types;

#[proc_macro_derive(Protocol, attributes(field))]
pub fn derive_protocol(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let mut marked_fields = Vec::new();

    let data = match &input.data {
        Data::Struct(data) => data,
        _ => {
            return syn::Error::new_spanned(input.ident, "Protocol only supports structs")
                .to_compile_error()
                .into();
        }
    };

    match &data.fields {
        Fields::Named(fields) => {
            for field in &fields.named {
                for attr in &field.attrs {
                    if attr.path().is_ident("field") {
                        match &attr.meta {
                            syn::Meta::List(list) => {
                                // parse list.tokens
                            }
                            syn::Meta::Path(_) => {
                                // #[field] (no args)
                            }
                            _ => {}
                        }
                    }
                }

                let has_attr = field.attrs.iter().any(|a| a.path().is_ident("field"));

                if has_attr {
                    let ident = field.ident.as_ref().unwrap();
                    let ty = &field.ty;
                    marked_fields.push((ident, ty));
                }
            }
        }
        _ => {
            return syn::Error::new_spanned(&data.fields, "Protocol requires named fields")
                .to_compile_error()
                .into();
        }
    }

    // Example codegen using the marked fields
    let field_names = marked_fields.iter().map(|(ident, _)| ident.to_string());

    let expanded = quote! {
        impl #struct_name {
            pub fn marked_fields(&self) -> &'static [&'static str] {
                &[#(#field_names),*]
            }
        }
    };

    expanded.into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, ItemImpl, parse_macro_input};

pub(crate) mod types;

#[proc_macro_derive(Protocol, attributes(field))]
pub fn derive_protocol(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let mut marked_fields: Vec<(syn::Ident, syn::Type)> = Vec::new();

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
                            syn::Meta::List(_list) => {
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
                    let ident = field.ident.clone().unwrap();
                    let ty = field.ty.clone();
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

    // let mut impl_block = parse_macro_input!(item as ItemImpl);
    //
    // let pdu_set_parent_method: syn::ImplItem = syn::parse_quote! {
    //     fn set_parent(&mut self, parent: Pob<'static>)
    //     where
    //         'a: 'static,
    //     {
    //         self.parent = parent;
    //     }
    // };
    //
    // impl_block.items.push(pdu_link_child_method);
    //
    // quote!(#impl_block).into()

    let mut fields_to_sum: Vec<syn::Type> = vec![];
    match input.clone().data {
        Data::Struct(mut data) => {
            fields_to_sum = data
                .fields
                .iter_mut()
                .filter_map(|f| {
                    let has_field_attr = f.attrs.iter().any(|attr| attr.path().is_ident("field"));
                    if has_field_attr {
                        Some(f.ty.clone())
                    } else {
                        None
                    }
                })
                .collect();
        }
        _ => (),
    };

    let field_names = marked_fields.iter().map(|(ident, _)| ident.to_string());

    let expanded = quote! {
        impl #struct_name {
            pub fn marked_fields(&self) -> &'static [&'static str] {
                &[#(#field_names),*]
            }

            pub fn total_width() -> usize {
                0 #( + #fields_to_sum::width() )*
            }
        }
    };

    // let mut ts = expanded.into();
    let mut gen_methods: Vec<syn::ItemImpl> = Vec::new();

    let mut summed_fields: Vec<syn::Type> = Vec::new();
    for (name, ty) in marked_fields.iter() {
        summed_fields.push(ty.clone());
        let field_getter: syn::ItemImpl = syn::parse_quote! {
            impl #struct_name {
                pub fn #name(&self) -> usize {
                    0 #( + #summed_fields::width() )*
                }
            }
        };

        gen_methods.push(field_getter);
    }

    quote! {
        # ( #gen_methods )*
        #expanded
    }
    .into()
}

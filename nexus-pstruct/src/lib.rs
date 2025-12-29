use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub(crate) mod types;

struct FieldMeta {
    skip: bool,
    pad_right: usize,
    pad_left: usize,
    repeat: Option<fn(&[u8]) -> usize>,
    enable: Option<fn(&[u8]) -> bool>,
    endian: String,
}

#[proc_macro_derive(Protocol, attributes(field, header))]
pub fn derive_protocol(input_stream: TokenStream) -> TokenStream {
    // let tokens = input_stream.clone();
    let input: DeriveInput = parse_macro_input!(input_stream as DeriveInput);

    let mut marked_fields: Vec<(syn::Ident, syn::Type)> = Vec::new();

    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

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
                    if attr.path().is_ident("header") {
                        continue;
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

    let base_impl_expanded = quote! {
        impl #generics #ident #generics #where_clause {
            pub fn marked_fields(&self) -> &'static [&'static str] {
                &[#(#field_names),*]
            }

            pub fn total_width() -> usize {
                0 #( + #fields_to_sum::width() )*
            }
        }
    };

    let mut gen_methods: Vec<syn::ItemImpl> = Vec::new();
    let mut summed_fields: Vec<syn::Type> = Vec::new();
    for (name, ty) in marked_fields.iter() {
        let field_set_fn_name = format_ident!("set_{}", name);
        let field_with_fn_name = format_ident!("with_{}", name);
        let field_get_fn_name = format_ident!("{}", name);
        let field_meta_fn_name = format_ident! {"__{}_metadata", name};

        let bit_offset_low_stmt: syn::Stmt = syn::parse_quote! {
            let bit_offset_low = 0 #( + #summed_fields::width() )*;
        };

        let bit_offset_high_stmt: syn::Stmt = syn::parse_quote! {
            let bit_offset_high = bit_offset_low + #ty::width();
        };

        let byte_offset_low_stmt: syn::Stmt = syn::parse_quote! {
            let byte_offset_low = bit_offset_low / 8;
        };

        let byte_offset_high_stmt: syn::Stmt = syn::parse_quote! {
            let byte_offset_high = bit_offset_high.div_ceil(8);
        };

        // let byte_boundary_panic_stmt: syn::Stmt = syn::parse_quote! {
        //     #byte_offset_low_stmt
        //     #byte_offset_high_stmt
        //     if (byte_offset_high - byte_offset_low) > #ty::width() {
        //         panic!("Byte slice sizes greater than type width is not currently supported");
        //     }
        // };

        // let bit_mask: syn::Stmt = syn::parse_quote! {};

        let field_get: syn::ItemImpl = syn::parse_quote! {
            impl #generics #ident #generics #where_clause {
                pub fn #field_get_fn_name(&self) -> #ty {
                    // 0 #( + #summed_fields::width() )*
                    self.#name
                }

                pub fn #field_meta_fn_name(&self) -> (usize, usize, usize, usize) {
                    #bit_offset_low_stmt
                    #bit_offset_high_stmt
                    #byte_offset_low_stmt
                    #byte_offset_high_stmt
                    (bit_offset_low, bit_offset_high, byte_offset_low, byte_offset_high)
                }
            }
        };

        summed_fields.push(ty.clone());
        gen_methods.push(field_get);
    }

    quote! {
        // #struct_expanded
        #base_impl_expanded
        # ( #gen_methods )*
    }
    .into()
}

#[proc_macro_attribute]
pub fn proto_base(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut struct_kind: syn::ItemStruct = syn::parse(item.clone()).unwrap();

    let pdu_fields: syn::FieldsNamed = syn::parse_quote!({
        __header: Cow<'a, [u8]>,
        __parent: Option<String>,
        __child: Option<String>,
    });

    if let syn::Fields::Named(fields) = &mut struct_kind.fields {
        fields.named.extend(pdu_fields.named);
    }

    quote!(#struct_kind).into()
}

#[proc_macro_attribute]
pub fn protocol(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut struct_kind: syn::ItemStruct = syn::parse(item.clone()).unwrap();
    let proto_base_attr: syn::Attribute = syn::parse_quote!(
        #[proto_base]
    );
    let derive_protocol: syn::Attribute = syn::parse_quote!(
        #[derive(Protocol)]
    );

    struct_kind.attrs.push(proto_base_attr);
    struct_kind.attrs.push(derive_protocol);

    quote!(#struct_kind).into()
}

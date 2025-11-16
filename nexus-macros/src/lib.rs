use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Tid)]
pub fn derive_tid(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = input.generics.clone();
    let (impl_g, ty_g, where_g) = generics.split_for_impl();

    let lifetime_param = input
        .generics
        .lifetimes()
        .next()
        .expect("Tid derive requires exactly one lifetime parameter");
    let lt = &lifetime_param.lifetime;

    let expanded = quote! {
        impl #impl_g Tid<#lt> for #name #ty_g #where_g {
            fn self_id(&self) -> TypeId {
                TypeId::of::<#name>()
            }

            fn id() -> TypeId {
                TypeId::of::<#name>()
            }
        }
    };

    expanded.into()
}

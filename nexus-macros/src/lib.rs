use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, ItemImpl, parse_macro_input};

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

#[proc_macro_attribute]
pub fn pdu_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut impl_block = parse_macro_input!(item as ItemImpl);

    let pdu_link_parent_method: syn::ImplItem = syn::parse_quote! {
        fn parent_pdu(&mut self) -> &mut Pob<'a> {
            &mut self.parent
        }
    };

    let pdu_link_child_method: syn::ImplItem = syn::parse_quote! {
        fn child_pdu(&mut self) -> &mut Pob<'a> {
            &mut self.child
        }
    };

    impl_block.items.push(pdu_link_parent_method);
    impl_block.items.push(pdu_link_child_method);

    quote!(#impl_block).into()
}

#[proc_macro_attribute]
pub fn pdu_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut struct_kind: syn::ItemStruct = syn::parse(item.clone()).unwrap();

    let pdu_fields: syn::FieldsNamed = syn::parse_quote!({
        header: Cow<'a, [u8]>,
        data: Cow<'a, [u8]>,
        parent: Pob<'a>,
        child: Pob<'a>,
    });

    if let syn::Fields::Named(fields) = &mut struct_kind.fields {
        fields.named.extend(pdu_fields.named);
    }

    let derive_tid: syn::Attribute = syn::parse_quote!(
        #[derive(Tid)]
    );

    struct_kind.attrs.push(derive_tid);

    quote!(#struct_kind).into()
}

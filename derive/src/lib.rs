mod types;

use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed, FieldsUnnamed, ItemStruct};

#[derive(Debug, FromDeriveInput)]
struct FieldOption {
    #[darling(default)]
    lowercase: bool,
}

#[proc_macro_derive(GQLField, attributes(field_name))]
pub fn test(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let option = FieldOption::from_derive_input(&item).unwrap();
    let struct_name = dbg!(item).ident;

    let gen = quote! {
        impl #struct_name {
            pub fn test(&self) -> &str {
                stringify!(#struct_name)
            }
        }
    };

    gen.into()
}

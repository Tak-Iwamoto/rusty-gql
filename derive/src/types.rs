use darling::{FromDeriveInput, FromField, FromMeta, FromVariant};
use syn::{Attribute, Generics, Ident};

#[derive(FromDeriveInput)]
#[darling(attributes(graphql), forward_attrs(doc))]
pub struct GqlData {
    pub ident: Ident,
    pub generics: Generics,
    pub attrs: Vec<Attribute>,

    #[darling(default)]
    pub field: Option<String>,
    #[darling(default)]
    pub parent_type: Option<String>,
}

pub struct GqlQuery {
    pub ident: Ident,
    pub generics: Generics,
    pub attrs: Vec<Attribute>,

    #[darling(default)]
    pub field: Option<String>,
    #[darling(default)]
    pub parent_type: Option<String>,
}

pub struct GqlMutation {
    pub ident: Ident,
    pub generics: Generics,
    pub attrs: Vec<Attribute>,

    #[darling(default)]
    pub field: Option<String>,
    #[darling(default)]
    pub parent_type: Option<String>,
}

pub struct GqlMutation {
    pub ident: Ident,
    pub generics: Generics,
    pub attrs: Vec<Attribute>,

    #[darling(default)]
    pub field: Option<String>,
    #[darling(default)]
    pub parent_type: Option<String>,
}

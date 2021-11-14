use darling::FromField;
use syn::{Attribute, Ident, Type, Visibility};

#[derive(FromField)]
#[darling(attributes(gql), forward_attrs(doc))]
pub struct GqlObject {
    pub ident: Option<Ident>,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ty: Type,
}

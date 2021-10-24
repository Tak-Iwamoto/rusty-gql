use darling::{FromField, FromMeta};
use syn::{Attribute, Ident, Type, Visibility};

#[derive(FromField)]
#[darling(attributes(gql), forward_attrs(doc))]
pub struct GqlObject {
    pub ident: Option<Ident>,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ty: Type,
}

// async-graphqlのproc_macro_attributeでもattributeの引数を取ることができる
// proc_macro_attributeのfieldごとのアノテーションはFromMetaで取得する
// #[Object(cache_control(max_age = 60))]

#[derive(FromMeta, Default)]
#[darling(default)]
pub struct GqlField {
    #[darling(default)]
    pub field: Option<String>,
    #[darling(default)]
    pub parent_type: Option<String>,
}

#[derive(FromMeta, Default)]
#[darling(default)]
pub struct GqlQuery {
    #[darling(default)]
    pub field: Option<String>,
}

#[derive(FromMeta, Default)]
#[darling(default)]
pub struct GqlMutation {
    #[darling(default)]
    pub field: Option<String>,
}

#[derive(FromMeta, Default)]
#[darling(default)]
pub struct GqlSubscription {
    #[darling(default)]
    pub field: Option<String>,
}

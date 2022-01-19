use graphql_parser::{schema::ObjectType, Pos};

use super::{directive::GqlDirective, field::GqlField};

#[derive(Debug, Clone)]
pub struct GqlObject {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub implements_interfaces: Vec<String>,
    pub directives: Vec<GqlDirective>,
    pub fields: Vec<GqlField>,
}

impl<'a> From<ObjectType<'a, String>> for GqlObject {
    fn from(object: ObjectType<'a, String>) -> Self {
        let directives = GqlDirective::from_vec_directive(object.directives);
        let fields = GqlField::from_vec_field(object.fields);

        GqlObject {
            name: object.name,
            description: object.description,
            position: object.position,
            implements_interfaces: object.implements_interfaces,
            directives,
            fields,
        }
    }
}

impl GqlObject {
    pub fn introspection_type() -> Self {
        GqlObject {
            name: "__Type".to_string(),
            description: Some("Requested Type information".to_string()),
            position: Pos::default(),
            directives: Default::default(),
            implements_interfaces: Default::default(),
            fields: Default::default(),
        }
    }
}

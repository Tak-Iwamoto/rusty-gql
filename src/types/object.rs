use graphql_parser::{schema::ObjectType as ParserObjectType, Pos};

use super::{directive::GqlDirective, field::FieldType};

#[derive(Debug, Clone)]
pub struct ObjectType {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub implements_interfaces: Vec<String>,
    pub directives: Vec<GqlDirective>,
    pub fields: Vec<FieldType>,
}

impl<'a> From<ParserObjectType<'a, String>> for ObjectType {
    fn from(object: ParserObjectType<'a, String>) -> Self {
        let directives = GqlDirective::from_vec_directive(object.directives);
        let fields = FieldType::from_vec_field(object.fields);

        ObjectType {
            name: object.name,
            description: object.description,
            position: object.position,
            implements_interfaces: object.implements_interfaces,
            directives,
            fields,
        }
    }
}

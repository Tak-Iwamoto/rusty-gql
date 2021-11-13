use graphql_parser::{schema::InterfaceType, Pos};

use super::{directive::GqlDirective, field::GqlField};

#[derive(Debug, Clone)]
pub struct GqlInterface {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
    pub fields: Vec<GqlField>,
}

impl<'a> From<InterfaceType<'a, String>> for GqlInterface {
    fn from(interface_type: InterfaceType<'a, String>) -> Self {
        let directives = GqlDirective::from_vec_directive(interface_type.directives);
        let fields = interface_type
            .fields
            .into_iter()
            .map(|field| GqlField::from(field))
            .collect();

        GqlInterface {
            name: interface_type.name,
            description: interface_type.description,
            position: interface_type.position,
            directives,
            fields,
        }
    }
}

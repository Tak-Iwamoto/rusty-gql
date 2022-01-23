use graphql_parser::{schema::InterfaceType as ParserInterfaceType, Pos};

use super::{directive::GqlDirective, field::FieldType};

#[derive(Debug, Clone)]
pub struct InterfaceType {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
    pub fields: Vec<FieldType>,
}

impl<'a> From<ParserInterfaceType<'a, String>> for InterfaceType {
    fn from(interface_type: ParserInterfaceType<'a, String>) -> Self {
        let directives = GqlDirective::from_vec_directive(interface_type.directives);
        let fields = interface_type
            .fields
            .into_iter()
            .map(|field| FieldType::from(field))
            .collect();

        InterfaceType {
            name: interface_type.name,
            description: interface_type.description,
            position: interface_type.position,
            directives,
            fields,
        }
    }
}

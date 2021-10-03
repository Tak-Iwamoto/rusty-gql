use graphql_parser::schema::ObjectType;

use super::{GraphQLDirective, GraphQLOperationSchema};

#[derive(Debug)]
pub struct GraphQLObjectType {
    name: String,
    description: Option<String>,
    fields: Vec<GraphQLOperationSchema>,
    directives: Vec<GraphQLDirective>,
    interfaces: Vec<String>,
}

impl GraphQLObjectType {
    pub fn parse<'a>(input: ObjectType<'a, &'a str>) -> Self {
        let interfaces = input
            .implements_interfaces
            .into_iter()
            .map(|interface| interface.to_string())
            .collect();
        let fields = input
            .fields
            .into_iter()
            .map(|field| GraphQLOperationSchema::parse(field))
            .collect();

        let directives = input
            .directives
            .into_iter()
            .map(|dir| GraphQLDirective::parse(dir))
            .collect();

        GraphQLObjectType {
            name: input.name.into(),
            description: input.description,
            fields,
            directives,
            interfaces,
        }
    }
}

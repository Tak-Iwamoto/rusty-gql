use graphql_parser::schema::InterfaceType;

use super::{GraphQLDirective, GraphQLField};

#[derive(Debug)]
pub struct GraphQLInterface {
    name: String,
    description: Option<String>,
    fields: Vec<GraphQLField>,
    directives: Vec<GraphQLDirective>,
}

impl GraphQLInterface {
    pub fn parse<'a>(input: InterfaceType<'a, &'a str>) -> Self {
        let fields = input
            .fields
            .into_iter()
            .map(|f| GraphQLField::parse(f))
            .collect();
        let directives = input
            .directives
            .into_iter()
            .map(|dir| GraphQLDirective::parse(dir))
            .collect();

        GraphQLInterface {
            name: input.name.to_string(),
            description: input.description,
            fields,
            directives,
        }
    }
}

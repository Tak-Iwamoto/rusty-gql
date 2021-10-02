use graphql_parser::schema::InterfaceType;

use super::GraphQLOperationSchema;

#[derive(Debug)]
pub struct GraphQLInterface {
    name: String,
    description: Option<String>,
    args: Vec<GraphQLOperationSchema>,
}

impl GraphQLInterface {
    pub fn parse<'a>(input: InterfaceType<'a, &'a str>) -> Self {
        let args = input
            .fields
            .into_iter()
            .map(|f| GraphQLOperationSchema::parse(f))
            .collect();

        GraphQLInterface {
            name: input.name.to_string(),
            description: input.description,
            args,
        }
    }
}

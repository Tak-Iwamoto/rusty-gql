use graphql_parser::schema::EnumType;

#[derive(Debug)]
pub struct GraphQLEnum {
    name: String,
    description: Option<String>,
    values: Vec<String>,
}

impl GraphQLEnum {
    pub fn parse<'a>(input: EnumType<'a, &'a str>) -> Self {
        // TODO: vにはdirectiveがあるが、一旦考慮していない
        let values: Vec<String> = input
            .values
            .into_iter()
            .map(|v| v.name.to_string())
            .collect();
        GraphQLEnum {
            name: input.name.to_string(),
            description: input.description,
            values,
        }
    }
}

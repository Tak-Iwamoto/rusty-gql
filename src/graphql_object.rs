use graphql_parser::schema::Field;

pub struct GraphQLObject<'a> {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field<'a, &'a str>>,
}

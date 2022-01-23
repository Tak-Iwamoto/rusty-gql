use super::directive::GqlDirective;
use graphql_parser::{schema::UnionType as ParserUnionType, Pos};

#[derive(Debug, Clone)]
pub struct UnionType {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
    pub types: Vec<String>,
}

impl<'a> From<ParserUnionType<'a, String>> for UnionType {
    fn from(gql_union: ParserUnionType<'a, String>) -> Self {
        let directives = GqlDirective::from_vec_directive(gql_union.directives);

        let types = gql_union.types;

        UnionType {
            name: gql_union.name,
            description: gql_union.description,
            position: gql_union.position,
            directives,
            types,
        }
    }
}

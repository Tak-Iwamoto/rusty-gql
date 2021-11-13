use super::directive::GqlDirective;
use graphql_parser::{schema::UnionType, Pos};

#[derive(Debug, Clone)]
pub struct GqlUnion {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
    pub types: Vec<String>,
}

impl<'a> From<UnionType<'a, String>> for GqlUnion {
    fn from(gql_union: UnionType<'a, String>) -> Self {
        let directives = GqlDirective::from_vec_directive(gql_union.directives);

        let types = gql_union.types;

        GqlUnion {
            name: gql_union.name,
            description: gql_union.description,
            position: gql_union.position,
            directives,
            types,
        }
    }
}

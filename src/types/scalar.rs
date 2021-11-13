use graphql_parser::{schema::ScalarType, Pos};

use super::directive::GqlDirective;

#[derive(Debug, Clone)]
pub struct GqlScalar {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
}

impl<'a> From<ScalarType<'a, String>> for GqlScalar {
    fn from(scalar_type: ScalarType<'a, String>) -> Self {
        let directives = GqlDirective::from_vec_directive(scalar_type.directives);
        GqlScalar {
            name: scalar_type.name,
            description: scalar_type.description,
            position: scalar_type.position,
            directives,
        }
    }
}

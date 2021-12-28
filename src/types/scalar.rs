use graphql_parser::{
    schema::{ScalarType, Value},
    Pos,
};

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

impl GqlScalar {
    pub fn is_valid_value(value: &Value<'_, String>) -> bool {
        match value {
            Value::Variable(_) => false,
            Value::Int(_) => true,
            Value::Float(_) => true,
            Value::String(_) => true,
            Value::Boolean(_) => true,
            Value::Null => true,
            Value::Enum(_) => false,
            Value::List(_) => false,
            Value::Object(_) => false,
        }
    }

    pub fn string_scalar() -> Self {
        GqlScalar {
            name: "String".to_string(),
            description: None,
            position: Pos::default(),
            directives: vec![],
        }
    }

    pub fn int_scalar() -> Self {
        GqlScalar {
            name: "Int".to_string(),
            description: None,
            position: Pos::default(),
            directives: vec![],
        }
    }

    pub fn float_scalar() -> Self {
        GqlScalar {
            name: "Float".to_string(),
            description: None,
            position: Pos::default(),
            directives: vec![],
        }
    }

    pub fn boolean_scalar() -> Self {
        GqlScalar {
            name: "Boolean".to_string(),
            description: None,
            position: Pos::default(),
            directives: vec![],
        }
    }

    pub fn id_scalar() -> Self {
        GqlScalar {
            name: "ID".to_string(),
            description: None,
            position: Pos::default(),
            directives: vec![],
        }
    }
}

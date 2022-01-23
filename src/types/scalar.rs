use graphql_parser::{
    schema::{ScalarType as ParserScalarType, Value},
    Pos,
};

use super::directive::GqlDirective;

#[derive(Debug, Clone)]
pub struct ScalarType {
    pub name: String,
    pub description: Option<String>,
    pub position: Pos,
    pub directives: Vec<GqlDirective>,
}

impl<'a> From<ParserScalarType<'a, String>> for ScalarType {
    fn from(scalar_type: ParserScalarType<'a, String>) -> Self {
        let directives = GqlDirective::from_vec_directive(scalar_type.directives);
        ScalarType {
            name: scalar_type.name,
            description: scalar_type.description,
            position: scalar_type.position,
            directives,
        }
    }
}

impl ScalarType {
    pub fn is_valid_value(&self, value: &Value<'_, String>) -> bool {
        match value {
            Value::Variable(_) => false,
            Value::Int(_) => self.name == "Int".to_string(),
            Value::Float(_) => self.name == "Float".to_string(),
            Value::String(_) => self.name == "String".to_string(),
            Value::Boolean(_) => self.name == "Boolean".to_string(),
            Value::Null => true,
            Value::Enum(_) => false,
            Value::List(_) => false,
            Value::Object(_) => false,
        }
    }

    pub fn string_scalar() -> Self {
        ScalarType {
            name: "String".to_string(),
            description: None,
            position: Pos::default(),
            directives: vec![],
        }
    }

    pub fn int_scalar() -> Self {
        ScalarType {
            name: "Int".to_string(),
            description: None,
            position: Pos::default(),
            directives: vec![],
        }
    }

    pub fn float_scalar() -> Self {
        ScalarType {
            name: "Float".to_string(),
            description: None,
            position: Pos::default(),
            directives: vec![],
        }
    }

    pub fn boolean_scalar() -> Self {
        ScalarType {
            name: "Boolean".to_string(),
            description: None,
            position: Pos::default(),
            directives: vec![],
        }
    }

    pub fn id_scalar() -> Self {
        ScalarType {
            name: "ID".to_string(),
            description: None,
            position: Pos::default(),
            directives: vec![],
        }
    }
}

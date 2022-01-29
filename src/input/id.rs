use crate::{types::ID, GqlValue};

use super::GqlInputType;

impl GqlInputType for ID {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::String(v) => Ok(ID(v)),
                GqlValue::Number(v) => Ok(ID(v.to_string())),
                invalid_value => Err(format!(
                    "{}: invalid gql value for id",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: id, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        GqlValue::String(self.0.clone())
    }
}

use crate::GqlValue;

use super::GqlInputType;

impl GqlInputType for bool {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Boolean(v) => Ok(v),
                invalid_value => Err(format!(
                    "Expected type: boolean, but found: {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: boolean, but not found".to_string()),
        }
    }

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::Boolean(*self)
    }
}

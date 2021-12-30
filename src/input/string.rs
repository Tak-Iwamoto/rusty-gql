use crate::GqlValue;

use super::GqlInputType;

impl GqlInputType for String {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::String(v) => Ok(v),
                invalid_value => Err(format!(
                    "{}: invalid gql value for string",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: boolean, but not found".to_string()),
        }
    }

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::String(self.clone())
    }
}

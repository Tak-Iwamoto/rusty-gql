use crate::GqlValue;

use super::GqlInputType;

impl<T: GqlInputType> GqlInputType for Option<T> {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value.unwrap_or_default() {
            GqlValue::Null => Ok(None),
            value => Ok(Some(T::from_gql_value(Some(value))?)),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        match self {
            Some(value) => value.to_gql_value(),
            None => GqlValue::Null,
        }
    }
}

use std::collections::{BTreeMap, HashMap};

use crate::GqlValue;

use super::GqlInputType;

impl<T: GqlInputType> GqlInputType for BTreeMap<String, T> {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Object(v) => {
                    let mut result = BTreeMap::new();
                    for (key, value) in v {
                        result.insert(key, T::from_gql_value(Some(value))?);
                    }
                    Ok(result)
                }
                invalid_value => Err(format!(
                    "Expected type: object, but found: {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: object, but not found".to_string()),
        }
    }

    fn into_gql_value(&self) -> GqlValue {
        let mut result = BTreeMap::new();
        for (key, value) in self {
            result.insert(key.clone(), T::into_gql_value(value.clone()));
        }
        GqlValue::Object(result)
    }
}

impl<T: GqlInputType> GqlInputType for HashMap<String, T> {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Object(v) => {
                    let mut result = HashMap::new();
                    for (key, value) in v {
                        result.insert(key, T::from_gql_value(Some(value))?);
                    }
                    Ok(result)
                }
                invalid_value => Err(format!(
                    "Expected type: object, but found: {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: object, but not found".to_string()),
        }
    }

    fn into_gql_value(&self) -> GqlValue {
        let mut result = BTreeMap::new();
        for (key, value) in self {
            result.insert(key.clone(), T::into_gql_value(value.clone()));
        }
        GqlValue::Object(result)
    }
}

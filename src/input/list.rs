use std::collections::{HashSet, LinkedList};
use std::convert::TryInto;
use std::hash::Hash;

use crate::GqlValue;

use super::GqlInputType;

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

#[async_trait::async_trait]
impl<T: GqlInputType, const N: usize> GqlInputType for [T; N] {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::List(list) => {
                    let mut result = Vec::new();
                    for v in list {
                        let value = T::from_gql_value(Some(v))?;
                        result.push(value)
                    }
                    Ok(vec_to_array(result))
                }
                invalid_value => Err(format!(
                    "Expected type: list, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: list, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        let values = self.into_iter().map(|v| v.to_gql_value()).collect();
        GqlValue::List(values)
    }
}

#[async_trait::async_trait]
impl<T: GqlInputType + Eq + Hash> GqlInputType for HashSet<T> {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::List(list) => {
                    let mut result = Vec::new();
                    for v in list {
                        let value = T::from_gql_value(Some(v))?;
                        result.push(value)
                    }
                    let hash_set: HashSet<T> = result.into_iter().collect();
                    Ok(hash_set)
                }
                invalid_value => Err(format!(
                    "Expected type: list, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: list, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        let values = self.into_iter().map(|v| v.to_gql_value()).collect();
        GqlValue::List(values)
    }
}

#[async_trait::async_trait]
impl<T: GqlInputType> GqlInputType for LinkedList<T> {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        todo!()
    }

    fn to_gql_value(&self) -> GqlValue {
        todo!()
    }
}

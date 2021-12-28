use serde_json::Number;

use crate::GqlValue;

use super::GqlInputType;

impl GqlInputType for i8 {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_i64().ok_or_else(|| "Invalid number".to_string())?;
                    Ok(n as Self)
                }
                invalid_value => Err(format!(
                    "Expected type: number, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: number, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl GqlInputType for i16 {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_i64().ok_or_else(|| "Invalid number".to_string())?;
                    Ok(n as Self)
                }
                invalid_value => Err(format!(
                    "Expected type: number, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: number, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl GqlInputType for i32 {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_i64().ok_or_else(|| "Invalid number".to_string())?;
                    Ok(n as Self)
                }
                invalid_value => Err(format!(
                    "Expected type: number, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: number, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl GqlInputType for i64 {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_i64().ok_or_else(|| "Invalid number".to_string())?;
                    Ok(n as Self)
                }
                invalid_value => Err(format!(
                    "Expected type: number, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: number, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl GqlInputType for u8 {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_i64().ok_or_else(|| "Invalid number".to_string())?;
                    Ok(n as Self)
                }
                invalid_value => Err(format!(
                    "Expected type: number, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: number, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl GqlInputType for u16 {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_i64().ok_or_else(|| "Invalid number".to_string())?;
                    Ok(n as Self)
                }
                invalid_value => Err(format!(
                    "Expected type: number, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: number, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl GqlInputType for u32 {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_i64().ok_or_else(|| "Invalid number".to_string())?;
                    Ok(n as Self)
                }
                invalid_value => Err(format!(
                    "Expected type: number, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: number, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl GqlInputType for u64 {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_i64().ok_or_else(|| "Invalid number".to_string())?;
                    Ok(n as Self)
                }
                invalid_value => Err(format!(
                    "Expected type: number, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: number, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl GqlInputType for usize {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_i64().ok_or_else(|| "Invalid number".to_string())?;
                    Ok(n as Self)
                }
                invalid_value => Err(format!(
                    "Expected type: number, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: number, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl GqlInputType for isize {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_i64().ok_or_else(|| "Invalid number".to_string())?;
                    Ok(n as Self)
                }
                invalid_value => Err(format!(
                    "Expected type: number, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: number, but not found".to_string()),
        }
    }

    fn to_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

// #[async_trait::async_trait]
// impl Resolver for f64 {
//     async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
//         Ok(Some(GqlValue::Number(
//             Number::from_f64(*self).unwrap_or_default(),
//         )))
//     }
// }

use serde_json::Number;

use crate::GqlValue;

use super::VariableType;

impl VariableType for i8 {
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

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl VariableType for i16 {
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

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl VariableType for i32 {
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

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl VariableType for i64 {
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

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl VariableType for u8 {
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

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl VariableType for u16 {
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

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl VariableType for u32 {
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

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl VariableType for u64 {
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

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl VariableType for usize {
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

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl VariableType for isize {
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

    fn into_gql_value(&self) -> GqlValue {
        GqlValue::Number(Number::from(*self))
    }
}

impl VariableType for f32 {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_f64().ok_or_else(|| "Invalid number".to_string())?;
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

    fn into_gql_value(&self) -> GqlValue {
        match Number::from_f64(*self as f64) {
            Some(n) => GqlValue::Number(n),
            None => GqlValue::Null,
        }
    }
}

impl VariableType for f64 {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::Number(v) => {
                    let n = v.as_f64().ok_or_else(|| "Invalid number".to_string())?;
                    Ok(n)
                }
                invalid_value => Err(format!(
                    "Expected type: number, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: number, but not found".to_string()),
        }
    }

    fn into_gql_value(&self) -> GqlValue {
        match Number::from_f64(*self) {
            Some(n) => GqlValue::Number(n),
            None => GqlValue::Null,
        }
    }
}

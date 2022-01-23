use rusty_gql_macro::GqlType;

use crate::{types::EnumTypeValue, SelectionSetResolver};

pub struct __EnumValue {
    detail: EnumTypeValue,
}

pub fn build_enum_value_introspection(value: &EnumTypeValue) -> __EnumValue {
    __EnumValue {
        detail: value.clone(),
    }
}

#[allow(non_snake_case)]
#[GqlType(internal)]
impl __EnumValue {
    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&str> {
        self.detail.description.as_deref()
    }

    async fn isDeprecated(&self) -> bool {
        self.detail.is_deprecated()
    }
}

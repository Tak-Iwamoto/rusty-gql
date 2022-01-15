use rusty_gql_macro::Resolver;

use crate::{types::GqlEnumValue, SelectionSetResolver};

pub struct __EnumValue {
    detail: GqlEnumValue,
}

pub fn build_enum_value_introspection(value: &GqlEnumValue) -> __EnumValue {
    __EnumValue {
        detail: value.clone(),
    }
}

#[Resolver(internal)]
impl __EnumValue {
    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&str> {
        self.detail.description.as_deref()
    }

    async fn is_deprecated(&self) -> bool {
        self.detail.is_deprecated()
    }
}

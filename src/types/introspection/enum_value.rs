use crate::{types::GqlEnumValue, Schema};

pub(crate) struct __EnumValue<'a> {
    schema: &'a Schema,
    detail: GqlEnumValue,
}

impl<'a> __EnumValue<'a> {
    pub fn new(schema: &'a Schema, value: &'a GqlEnumValue) -> Self {
        Self {
            schema,
            detail: value.clone(),
        }
    }

    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&String> {
        self.detail.description.as_ref()
    }

    async fn is_deprecated(&self) -> bool {
        self.detail.is_deprecated()
    }
}

use graphql_parser::schema::DirectiveLocation;

use crate::{
    types::GqlDirectiveDefinition, FieldContext, GqlValue, Resolver, ResolverResult, Schema,
    SelectionSetResolver,
};

use super::input_value::__InputValue;

pub(crate) struct __Directive<'a> {
    pub schema: &'a Schema,
    pub detail: &'a GqlDirectiveDefinition,
}

fn dir_location_str(location: &DirectiveLocation) -> String {
    match location {
        DirectiveLocation::Query => "QUERY".to_string(),
        DirectiveLocation::Mutation => "MUTATION".to_string(),
        DirectiveLocation::Subscription => "SUBSCRIPTION".to_string(),
        DirectiveLocation::Field => "FIELD".to_string(),
        DirectiveLocation::FragmentDefinition => "FRAGMENT_DEFINITION".to_string(),
        DirectiveLocation::FragmentSpread => "FRAGMENT_SPREAD".to_string(),
        DirectiveLocation::InlineFragment => "INLINE_FRAGMENT".to_string(),
        DirectiveLocation::Schema => "SCHEMA".to_string(),
        DirectiveLocation::Scalar => "SCALAR".to_string(),
        DirectiveLocation::Object => "OBJECT".to_string(),
        DirectiveLocation::FieldDefinition => "FIELD_DEFINITION".to_string(),
        DirectiveLocation::ArgumentDefinition => "ARGUMENT_DEFINITION".to_string(),
        DirectiveLocation::Interface => "INTERFACE".to_string(),
        DirectiveLocation::Union => "UNION".to_string(),
        DirectiveLocation::Enum => "ENUM".to_string(),
        DirectiveLocation::EnumValue => "ENUM_VALUE".to_string(),
        DirectiveLocation::InputObject => "INPUT_OBJECT".to_string(),
        DirectiveLocation::InputFieldDefinition => "INPUT_FIELD_DEFINITION".to_string(),
    }
}

impl<'a> __Directive<'a> {
    pub fn new(schema: &'a Schema, directive: &'a GqlDirectiveDefinition) -> Self {
        __Directive {
            schema,
            detail: directive,
        }
    }
    async fn name(&self) -> &str {
        self.detail.name.as_str()
    }

    async fn description(&self) -> Option<&String> {
        self.detail.description.as_ref()
    }

    async fn locations(&self) -> &Vec<DirectiveLocation> {
        &self.detail.locations
    }

    async fn args(&'a self) -> Vec<__InputValue<'a>> {
        let mut result = Vec::new();

        for arg in &self.detail.arguments {
            let value = __InputValue::new(self.schema, arg);
            result.push(value);
        }
        result
    }
}

#[async_trait::async_trait]
impl<'a> Resolver for __Directive<'a> {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        if ctx.item.name == "name" {
            let name = self.name().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(name, &ctx_selection_set)
                .await
                .map(Some);
        }

        if ctx.item.name == "description" {
            let desc = self.description().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match desc {
                Some(v) => {
                    return SelectionSetResolver::resolve_selection_set(v, &ctx_selection_set)
                        .await
                        .map(Some);
                }
                None => return Ok(None),
            }
        }

        if ctx.item.name == "locations" {
            let locations = self.locations().await;
            let locs: Vec<String> = locations.iter().map(|loc| dir_location_str(loc)).collect();
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(&locs, &ctx_selection_set)
                .await
                .map(Some);
        }

        if ctx.item.name == "args" {
            let args = self.args().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(&args, &ctx_selection_set)
                .await
                .map(Some);
        }

        Ok(None)
    }
}

#[async_trait::async_trait]
impl<'a> SelectionSetResolver for __Type<'a> {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        ctx.resolve_selection_parallelly(self).await
    }
}

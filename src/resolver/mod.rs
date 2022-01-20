mod boolean;
mod list;
mod number;
mod object;
mod optional;
mod string;

use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use futures_util::{
    future::{try_join_all, BoxFuture},
    FutureExt,
};
use graphql_parser::query::{Selection, TypeCondition};

use crate::{
    context::{FieldContext, SelectionSetContext},
    GqlDirective, GqlError, GqlTypeDefinition, GqlValue, ResolverResult,
};

#[async_trait]
pub trait SelectionSetResolver: FieldResolver {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue>;
}

#[async_trait]
pub trait FieldResolver: Send + Sync {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>>;

    fn type_name() -> String;

    fn introspection_type_name(&self) -> String {
        Self::type_name()
    }

    fn collect_all_fields<'a, 'ctx: 'a>(
        &'a self,
        ctx: &SelectionSetContext<'ctx>,
        fields: &mut Fields<'a>,
    ) -> ResolverResult<()> {
        fields.collect_fields(ctx, self)
    }
}

#[async_trait::async_trait]
impl<T: FieldResolver> FieldResolver for &T {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        T::resolve_field(*self, ctx).await
    }
    fn type_name() -> String {
        T::type_name()
    }
}

#[async_trait::async_trait]
impl<T: FieldResolver> FieldResolver for Arc<T> {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        T::resolve_field(self, ctx).await
    }
    fn type_name() -> String {
        T::type_name()
    }
}

#[async_trait::async_trait]
impl<T: FieldResolver> FieldResolver for Box<T> {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        T::resolve_field(self, ctx).await
    }
    fn type_name() -> String {
        T::type_name()
    }
}

pub async fn resolve_selection_parallelly<'a, 'ctx: 'a, T: FieldResolver + SelectionSetResolver>(
    ctx: &SelectionSetContext<'ctx>,
    root_type: &'a T,
) -> ResolverResult<GqlValue> {
    resolve_selection(ctx, root_type, true).await
}

pub async fn resolve_selection_serially<'a, 'ctx: 'a, T: FieldResolver + SelectionSetResolver>(
    ctx: &SelectionSetContext<'ctx>,
    root_type: &'a T,
) -> ResolverResult<GqlValue> {
    resolve_selection(ctx, root_type, false).await
}
async fn resolve_selection<'a, 'ctx: 'a, T: FieldResolver + SelectionSetResolver>(
    ctx: &SelectionSetContext<'ctx>,
    root_type: &'a T,
    parallel: bool,
) -> ResolverResult<GqlValue> {
    let mut fields = Fields(Vec::new());
    fields.collect_fields(ctx, root_type)?;

    let res = if parallel {
        try_join_all(fields.0).await?
    } else {
        let mut results = Vec::new();
        for resolver in fields.0 {
            results.push(resolver.await?);
        }
        results
    };

    let mut gql_obj_map = BTreeMap::new();

    for value in res {
        build_gql_object(&mut gql_obj_map, value);
    }

    Ok(GqlValue::Object(gql_obj_map))
}

fn build_gql_object(target_obj: &mut BTreeMap<String, GqlValue>, gql_value: (String, GqlValue)) {
    let (field_name, value) = gql_value;
    if let Some(prev_value) = target_obj.get_mut(&field_name) {
        match prev_value {
            GqlValue::List(target_list) => {
                if let GqlValue::List(list) = value {
                    for (index, v) in list.into_iter().enumerate() {
                        if let Some(prev_value) = target_list.get_mut(index) {
                            if let GqlValue::Object(prev_obj) = prev_value {
                                if let GqlValue::Object(new_obj) = v {
                                    for (key, value) in new_obj.into_iter() {
                                        build_gql_object(prev_obj, (key, value))
                                    }
                                }
                            }
                        }
                    }
                }
            }
            GqlValue::Object(prev_obj) => {
                if let GqlValue::Object(obj) = value {
                    for map in obj.into_iter() {
                        build_gql_object(prev_obj, (map.0, map.1))
                    }
                }
            }
            _ => return,
        }
    } else {
        target_obj.insert(field_name, value.clone());
    }
}

pub type ResolveFieldFuture<'a> = BoxFuture<'a, ResolverResult<(String, GqlValue)>>;
pub struct Fields<'a>(Vec<ResolveFieldFuture<'a>>);

impl<'a> Fields<'a> {
    pub fn collect_fields<'ctx: 'a, T: FieldResolver + ?Sized>(
        &mut self,
        ctx: &SelectionSetContext<'ctx>,
        root_type: &'a T,
    ) -> ResolverResult<()> {
        for item in &ctx.item.items {
            match &item {
                Selection::Field(field) => {
                    if ctx.is_skip(&field.directives) {
                        continue;
                    }
                    if field.name == "__typename" {
                        ctx.with_field(field);
                        let field_name = field.name.clone();
                        let type_name = root_type.introspection_type_name();

                        self.0.push(Box::pin(async move {
                            Ok((field_name, GqlValue::String(type_name)))
                        }));
                        continue;
                    }

                    self.0.push(Box::pin({
                        let ctx = ctx.clone();
                        async move {
                            let ctx_field = &ctx.with_field(field);
                            let field_name = ctx_field.item.name.clone();
                            let type_name = T::type_name();
                            let empty_vec = vec![];

                            let query_directives = &field.directives;
                            let schema_ty_directives = ctx
                                .schema
                                .type_definitions
                                .get(&type_name)
                                .map(|ty_def| ty_def.directives())
                                .unwrap_or(empty_vec.as_slice());
                            let schema_field_directives = ctx
                                .schema
                                .type_definitions
                                .get(&type_name)
                                .map(|ty_def| ty_def.field_directives(&field_name))
                                .unwrap_or(vec![]);
                            let schema_impl_interface_directives = ctx
                                .schema
                                .type_definitions
                                .get(&type_name)
                                .map(|ty_def| ty_def.impl_interface_directives(&ctx.schema))
                                .unwrap_or(vec![]);
                            let resolve_fut = root_type.resolve_field(&ctx_field);

                            if schema_ty_directives.is_empty()
                                && schema_field_directives.is_empty()
                                && schema_impl_interface_directives.is_empty()
                                && query_directives.is_empty()
                            {
                                Ok((
                                    field_name,
                                    root_type
                                        .resolve_field(&ctx_field)
                                        .await?
                                        .unwrap_or_default(),
                                ))
                            } else {
                                let mut resolve_fut = resolve_fut.boxed();

                                for directive in query_directives {
                                    if let Some(custom_dir) =
                                        ctx.schema.custom_directives.get(directive.name.as_str())
                                    {
                                        resolve_fut = Box::pin({
                                            let directive = GqlDirective::from(directive.clone());
                                            let ctx = ctx_field.clone();
                                            async move {
                                                custom_dir
                                                    .resolve_field(
                                                        &ctx,
                                                        &directive.arguments,
                                                        &mut resolve_fut,
                                                    )
                                                    .await
                                            }
                                        })
                                    }
                                }

                                for directive in schema_ty_directives {
                                    if let Some(custom_dir) =
                                        ctx.schema.custom_directives.get(directive.name.as_str())
                                    {
                                        resolve_fut = Box::pin({
                                            let ctx = ctx_field.clone();
                                            async move {
                                                custom_dir
                                                    .resolve_field(
                                                        &ctx,
                                                        &directive.arguments,
                                                        &mut resolve_fut,
                                                    )
                                                    .await
                                            }
                                        })
                                    }
                                }

                                for directive in &schema_field_directives {
                                    if let Some(custom_dir) =
                                        ctx.schema.custom_directives.get(directive.name.as_str())
                                    {
                                        resolve_fut = Box::pin({
                                            let ctx = ctx_field.clone();
                                            async move {
                                                custom_dir
                                                    .resolve_field(
                                                        &ctx,
                                                        &directive.arguments,
                                                        &mut resolve_fut,
                                                    )
                                                    .await
                                            }
                                        })
                                    }
                                }

                                for directive in &schema_impl_interface_directives {
                                    if let Some(custom_dir) =
                                        ctx.schema.custom_directives.get(directive.name.as_str())
                                    {
                                        resolve_fut = Box::pin({
                                            let ctx = ctx_field.clone();
                                            async move {
                                                custom_dir
                                                    .resolve_field(
                                                        &ctx,
                                                        &directive.arguments,
                                                        &mut resolve_fut,
                                                    )
                                                    .await
                                            }
                                        })
                                    }
                                }
                                Ok((field_name, resolve_fut.await?.unwrap_or_default()))
                            }
                        }
                    }))
                }
                Selection::FragmentSpread(fragment_spread) => {
                    let operation_fragment = ctx
                        .operation
                        .fragment_definitions
                        .get(&fragment_spread.fragment_name);
                    let fragment_def = match operation_fragment {
                        Some(fragment) => fragment,
                        None => {
                            return Err(GqlError::new(
                                format!("{:?} is not defined in query", fragment_spread),
                                Some(fragment_spread.position),
                            ))
                        }
                    };

                    if is_fragment_condition(
                        ctx,
                        &root_type.introspection_type_name(),
                        Some(&fragment_def.type_condition),
                    ) {
                        root_type.collect_all_fields(
                            &ctx.with_selection_set(&fragment_def.selection_set),
                            self,
                        )?;
                    }
                }
                Selection::InlineFragment(inline_fragment) => {
                    if ctx.is_skip(&inline_fragment.directives) {
                        continue;
                    }

                    if is_fragment_condition(
                        ctx,
                        &root_type.introspection_type_name(),
                        inline_fragment.type_condition.as_ref(),
                    ) {
                        root_type.collect_all_fields(
                            &ctx.with_selection_set(&inline_fragment.selection_set),
                            self,
                        )?;
                    } else if inline_fragment.type_condition.is_none() {
                        self.collect_fields(
                            &ctx.with_selection_set(&inline_fragment.selection_set),
                            root_type,
                        )?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn is_fragment_condition<'a, 'ctx: 'a>(
    ctx: &SelectionSetContext<'ctx>,
    type_name: &String,
    ty_cond: Option<&TypeCondition<'a, String>>,
) -> bool {
    match ty_cond {
        Some(cond) => {
            let on_type = match cond {
                TypeCondition::On(ty) => ty,
            };
            let is_on_type_name = on_type == type_name;
            let is_impl_interface =
                ctx.schema
                    .type_definitions
                    .get(type_name)
                    .map_or(false, |ty_def| {
                        if let GqlTypeDefinition::Object(obj) = ty_def {
                            obj.implements_interfaces.contains(on_type)
                        } else {
                            false
                        }
                    });
            is_on_type_name || is_impl_interface
        }
        None => false,
    }
}

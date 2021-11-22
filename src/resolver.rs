use std::collections::{BTreeMap, HashMap};

use async_trait::async_trait;
use futures::future::BoxFuture;
use graphql_parser::{
    query::{Field, Selection, SelectionSet},
    schema::Type,
};

use crate::{
    context::ExecutionContext, error::GqlError, operation::Operation, types::value::value_from_ast,
    GqlType, GqlValue, Response, Schema,
};

type ResolverFuture<'a> = BoxFuture<'a, Response<(String, GqlValue)>>;

#[async_trait]
pub trait Resolver: Send + Sync {
    async fn resolve(&self, ctx: &ExecutionContext<'_>) -> Response<Option<GqlValue>>;
}

#[async_trait::async_trait]
impl<T: Resolver> Resolver for &T {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn resolve(&self, ctx: &ExecutionContext<'_>) -> Response<Option<GqlValue>> {
        T::resolve(*self, ctx).await
    }
}

pub(crate) async fn resolve_query<'a, T: Resolver + ?Sized>(
    ctx: &'a ExecutionContext<'a>,
    query_resolver: &'a T,
) -> Response<GqlValue> {
    resolve_object(query_resolver, ctx, true).await
}

pub(crate) async fn resolve_mutation<'a, T: Resolver + ?Sized>(
    ctx: &'a ExecutionContext<'a>,
    mutation_resolver: &'a T,
) -> Response<GqlValue> {
    resolve_object(mutation_resolver, ctx, false).await
}

pub(crate) async fn resolve_subscription<'a, T: Resolver + ?Sized>(
    ctx: &'a ExecutionContext<'a>,
    subscription_resolver: &'a T,
) -> Response<GqlValue> {
    resolve_object(subscription_resolver, ctx, false).await
}

fn build_gql_object(target_obj: &mut BTreeMap<String, GqlValue>, gql_value: (String, GqlValue)) {
    let (field_name, value) = gql_value;
    if let Some(prev_value) = target_obj.get_mut(&field_name) {
        match prev_value {
            GqlValue::List(target_list) => {
                if let GqlValue::List(list) = value {
                    for (index, v) in list.into_iter().enumerate() {
                        match target_list.get_mut(index) {
                            Some(prev_value) => {
                                if let GqlValue::Object(prev_obj) = prev_value {
                                    if let GqlValue::Object(new_obj) = v {
                                        for (key, value) in new_obj.into_iter() {
                                            build_gql_object(prev_obj, (key, value))
                                        }
                                    }
                                }
                            }
                            None => todo!(),
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

pub struct Resolvers<'a>(Vec<ResolverFuture<'a>>);

pub async fn resolve_object<'a, T: Resolver + ?Sized>(
    parent_type: &'a T,
    ctx: &ExecutionContext<'a>,
    parallel: bool,
) -> Response<GqlValue> {
    let mut resolvers = Resolvers(Vec::new());

    resolvers.collect_field_resolvers(parent_type, ctx, &ctx.operation.selection_set)?;

    let res = if parallel {
        futures::future::try_join_all(resolvers.0).await?
    } else {
        let mut results = Vec::new();
        for resolver in resolvers.0 {
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

impl<'a> Resolvers<'a> {
    pub fn collect_field_resolvers<T: Resolver + ?Sized>(
        &mut self,
        parent_type: &'a T,
        ctx: &ExecutionContext<'a>,
        selection_set: &'a SelectionSet<'a, String>,
    ) -> Response<()> {
        for item in &selection_set.items {
            match item {
                Selection::Field(field) => {
                    if ctx.is_skip(&field.directives) {
                        continue;
                    }
                    self.0.push(Box::pin({
                        let ctx = ctx.clone();
                        async move {
                            ctx.current_field(field.clone());
                            let field_name = &field.name;
                            Ok((
                                field_name.clone(),
                                parent_type.resolve(&ctx).await?.unwrap_or_default(),
                            ))
                        }
                    }))
                }
                Selection::FragmentSpread(fragment_spread) => {
                    let operation_fragment =
                        ctx.operation.fragments.get(&fragment_spread.fragment_name);
                    let fragment_def = match operation_fragment {
                        Some(fragment) => fragment,
                        None => {
                            return Err(GqlError::new(
                                format!("{:?} is not found in operation", fragment_spread),
                                Some(fragment_spread.position),
                            ))
                        }
                    };
                    self.collect_field_resolvers(parent_type, ctx, &fragment_def.selection_set)?;
                }
                Selection::InlineFragment(inline_fragment) => {
                    if ctx.is_skip(&inline_fragment.directives) {
                        continue;
                    }
                    self.collect_field_resolvers(parent_type, ctx, &inline_fragment.selection_set)?;
                }
            }
        }
        Ok(())
    }
}

pub fn get_variables<'a>(
    schema: &'a Schema,
    operation: &'a Operation<'a>,
    input_values: &BTreeMap<String, GqlValue>,
) -> Result<HashMap<String, GqlValue>, String> {
    let variable_definitions = &operation.variable_definitions;
    let mut variables = HashMap::new();
    for var in variable_definitions {
        let var_type = get_type_from_schema(schema, &var.var_type);
        if var_type.is_none() {
            continue;
        }
        let var_type = var_type.unwrap();

        let var_name = &var.name.to_string();
        if !input_values.contains_key(var_name) {
            if let Some(value) = &var.default_value {
                variables.insert(
                    var.name.to_string(),
                    value_from_ast(value, &var_type, &None),
                );
            }
        }

        let value = input_values.get(var_name);

        if let GqlType::NonNull(_) = var_type {
            if value.is_none() {
                return Err(format!("{} must not be null", var_name));
            }
        }

        if let Some(var_value) = value {
            variables.insert(var_name.to_string(), var_value.clone());
        }
    }
    Ok(variables)
}

pub fn get_arguments<'a>(field: Field<'a, String>, variable_values: HashMap<String, GqlValue>) {
    let arguments = field.arguments;
}

pub fn get_type_from_schema<'a>(
    schema: &'a Schema,
    var_type: &'a Type<'a, String>,
) -> Option<GqlType> {
    match var_type {
        graphql_parser::schema::Type::NamedType(named_type) => {
            return schema
                .type_map
                .get(&named_type.to_string())
                .map(|var_ty| var_ty.clone())
        }
        graphql_parser::schema::Type::ListType(list) => {
            let inner_type = get_type_from_schema(schema, &list).unwrap();
            let value = GqlType::List(Box::new(inner_type.clone()));
            return Some(value);
        }
        graphql_parser::schema::Type::NonNullType(non_null) => {
            let inner_type = get_type_from_schema(schema, &non_null).unwrap();
            let value = GqlType::NonNull(Box::new(inner_type.clone()));
            return Some(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        context::build_context,
        operation::{build_operation, ArcOperation},
        types::schema::{build_schema, ArcSchema},
    };
    use std::fs;

    #[test]
    fn it_works() {
        let schema_doc = fs::read_to_string("src/tests/github.graphql").unwrap();
        let query_doc = fs::read_to_string("src/tests/github_query.graphql").unwrap();

        let schema = ArcSchema::new(build_schema(schema_doc.as_str()).unwrap());
        let query = build_operation(query_doc.as_str(), &schema, None).unwrap();

        let operation = ArcOperation::new(query);
        let context = build_context(&schema, &operation);
    }
}

use futures_util::future::{try_join_all, BoxFuture};
use std::collections::BTreeMap;

use crate::{
    error::GqlError, input::GqlInputType, operation::ArcOperation, path::GraphQLPath,
    types::schema::ArcSchema, FieldResolver, GqlTypeDefinition, GqlValue, ResolverResult,
};
use graphql_parser::{
    query::{Field, Selection, SelectionSet},
    schema::{Directive, Value},
};

pub type ResolverFuture<'a> = BoxFuture<'a, ResolverResult<(String, GqlValue)>>;

#[derive(Debug, Clone)]
pub struct ExecutionContext<'a, T> {
    pub schema: &'a ArcSchema,
    pub operation: &'a ArcOperation<'a>,
    pub item: T,
    pub current_path: GraphQLPath,
}

pub type FieldContext<'a> = ExecutionContext<'a, &'a Field<'a, String>>;

impl<'a> FieldContext<'a> {
    pub fn get_arg_value<T: GqlInputType>(&self, arg_name: &str) -> ResolverResult<T> {
        let value = self
            .item
            .arguments
            .iter()
            .find(|(name, _)| name == arg_name)
            .map(|(_, v)| v);
        let gql_value = match value {
            Some(v) => {
                if let Value::Variable(var_name) = v {
                    self.resolve_variable_value(var_name)?
                } else {
                    GqlValue::from(v.clone())
                }
            }
            None => GqlValue::Null,
        };
        match T::from_gql_value(Some(gql_value)) {
            Ok(v) => Ok(v),
            Err(err) => Err(GqlError::new(err, None)),
        }
    }
}

pub type SelectionSetContext<'a> = ExecutionContext<'a, &'a SelectionSet<'a, String>>;

impl<'a, T> ExecutionContext<'a, T> {
    pub fn with_field(
        &self,
        field: &'a Field<'a, String>,
    ) -> ExecutionContext<'a, &'a Field<'a, String>> {
        ExecutionContext {
            schema: self.schema,
            operation: self.operation,
            item: field,
            current_path: self.current_path.clone(),
        }
    }

    pub fn with_selection_set(
        &self,
        selection_set: &'a SelectionSet<'a, String>,
    ) -> ExecutionContext<'a, &'a SelectionSet<'a, String>> {
        ExecutionContext {
            schema: self.schema,
            operation: self.operation,
            item: selection_set,
            current_path: self.current_path.clone(),
        }
    }

    pub fn is_skip(&self, directives: &'a [Directive<'a, String>]) -> bool {
        for dir in directives {
            let skip = match dir.name.as_str() {
                "skip" => true,
                "include" => false,
                _ => continue,
            };

            for (key, value) in &dir.arguments {
                if key != "if" {
                    continue;
                } else {
                    if let Value::Boolean(cond) = value {
                        if skip && *cond {
                            return true;
                        }
                    }
                }
            }

            return skip;
        }

        false
    }
    pub fn add_error(&self, error: &GqlError) {
        self.operation.errors.lock().unwrap().push(error.clone());
    }

    pub fn resolve_variable_value(&self, name: &str) -> ResolverResult<GqlValue> {
        let v = self
            .operation
            .variable_definitions
            .iter()
            .find(|var_def| var_def.name == name)
            .and_then(|var_def| self.operation.variables.0.get(&var_def.name));
        match v {
            Some(value) => Ok(value.clone()),
            None => Err(GqlError::new(
                format!("Variable {} is not defined", name),
                None,
            )),
        }
    }
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

impl<'a> SelectionSetContext<'a> {
    pub async fn resolve_selection_parallelly<'b, T: FieldResolver>(
        &'b self,
        root_type: &'b T,
    ) -> ResolverResult<GqlValue> {
        self.resolve_selection(root_type, true).await
    }

    pub async fn resolve_selection_serially<'b, T: FieldResolver>(
        &'b self,
        root_type: &'b T,
    ) -> ResolverResult<GqlValue> {
        self.resolve_selection(root_type, false).await
    }

    async fn resolve_selection<'b, T: FieldResolver>(
        &'b self,
        root_type: &'b T,
        parallel: bool,
    ) -> ResolverResult<GqlValue> {
        let resolvers = self.collect_fields(root_type)?;

        let res = if parallel {
            try_join_all(resolvers).await?
        } else {
            let mut results = Vec::new();
            for resolver in resolvers {
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

    pub fn collect_fields<'b, T: FieldResolver>(
        &'b self,
        root_type: &'b T,
    ) -> ResolverResult<Vec<ResolverFuture<'b>>> {
        let mut resolvers: Vec<ResolverFuture<'b>> = Vec::new();
        for item in &self.item.items {
            match &item {
                Selection::Field(field) => {
                    if self.is_skip(&field.directives) {
                        continue;
                    }
                    if field.name == "__typename" {
                        let field_name = field.name.clone();
                        let type_name = match self.schema.type_definitions.get(&field_name) {
                            Some(type_def) => type_def.name(),
                            None => "Null",
                        };

                        resolvers.push(Box::pin(async move {
                            Ok((field_name, GqlValue::String(type_name.to_string())))
                        }));
                        continue;
                    }

                    resolvers.push(Box::pin({
                        let ctx = self.clone();
                        async move {
                            let ctx_field = &ctx.with_field(field);
                            let field_name = ctx_field.item.name.clone();
                            Ok((
                                field_name,
                                root_type
                                    .resolve_field(&ctx_field)
                                    .await?
                                    .unwrap_or_default(),
                            ))
                        }
                    }))
                }
                Selection::FragmentSpread(fragment_spread) => {
                    let operation_fragment = self
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
                    let on_type = match &fragment_def.type_condition {
                        graphql_parser::query::TypeCondition::On(ty) => ty,
                    };
                    let type_name = T::type_name();

                    let is_on_type_name = on_type == &type_name;
                    let is_impl_interface =
                        self.schema
                            .type_definitions
                            .get(&type_name)
                            .map_or(false, |ty_def| {
                                if let GqlTypeDefinition::Object(obj) = ty_def {
                                    obj.implements_interfaces.contains(on_type)
                                } else {
                                    false
                                }
                            });
                    if is_on_type_name || is_impl_interface {
                        self.with_selection_set(&fragment_def.selection_set)
                            .collect_fields(root_type)?;
                    }
                }
                Selection::InlineFragment(inline_fragment) => {
                    if self.is_skip(&inline_fragment.directives) {
                        continue;
                    }
                    let on_type_str = match &inline_fragment.type_condition {
                        Some(ty) => match ty {
                            graphql_parser::query::TypeCondition::On(on_ty) => Some(on_ty),
                        },
                        None => None,
                    };
                    match on_type_str {
                        Some(on_type) => {
                            let type_name = T::type_name();

                            let is_on_type_name = on_type == &type_name;
                            let is_impl_interface = self
                                .schema
                                .type_definitions
                                .get(&type_name)
                                .map_or(false, |ty_def| {
                                    if let GqlTypeDefinition::Object(obj) = ty_def {
                                        obj.implements_interfaces.contains(on_type)
                                    } else {
                                        false
                                    }
                                });
                            if is_on_type_name || is_impl_interface {
                                self.with_selection_set(&inline_fragment.selection_set)
                                    .collect_fields(root_type)?;
                            }
                        }
                        None => {
                            self.with_selection_set(&inline_fragment.selection_set)
                                .collect_fields(root_type)?;
                        }
                    }
                }
            }
        }
        Ok(resolvers)
    }
}

pub(crate) fn build_context<'a>(
    schema: &'a ArcSchema,
    operation: &'a ArcOperation<'a>,
) -> ExecutionContext<'a, &'a SelectionSet<'a, String>> {
    let operation_type = operation.operation_type.to_string();

    let current_path = GraphQLPath::default()
        .prev(None)
        .parent_name(operation_type);

    ExecutionContext {
        schema,
        operation,
        item: &operation.selection_set,
        current_path,
    }
}

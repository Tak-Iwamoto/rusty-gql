use std::collections::BTreeMap;

use crate::{
    error::GqlError, operation::ArcOperation, path::GraphQLPath, resolver::ResolverFuture,
    types::schema::ArcSchema, GqlValue, Resolver, Response,
};
use graphql_parser::{
    query::{Field, Selection, SelectionSet},
    schema::Directive,
};

#[derive(Debug, Clone)]
pub struct ExecutionContext<'a, T> {
    pub schema: &'a ArcSchema,
    pub operation: &'a ArcOperation<'a>,
    pub item: T,
    pub current_path: GraphQLPath,
    pub errors: Vec<GqlError>,
}

pub type FieldContext<'a> = ExecutionContext<'a, &'a Field<'a, String>>;

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
            errors: self.errors.clone(),
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
            errors: self.errors.clone(),
        }
    }

    pub fn is_skip(&self, directives: &'a [Directive<'a, String>]) -> bool {
        for dir in directives {
            let skip = match dir.name.as_str() {
                "skip" => true,
                "include" => false,
                _ => continue,
            };
            return skip;
        }
        false
    }
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

impl<'a> SelectionSetContext<'a> {
    pub async fn resolve_selection_parallelly<'b, T: Resolver>(
        &'b self,
        parent_type: &'b T,
    ) -> Response<GqlValue> {
        self.resolve_selection(parent_type, true).await
    }

    pub async fn resolve_selection_serially<'b, T: Resolver>(
        &'b self,
        parent_type: &'b T,
    ) -> Response<GqlValue> {
        self.resolve_selection(parent_type, false).await
    }

    async fn resolve_selection<'b, T: Resolver>(
        &'b self,
        parent_type: &'b T,
        parallel: bool,
    ) -> Response<GqlValue> {
        let resolvers = self.collect_fields(parent_type)?;

        let res = if parallel {
            futures::future::try_join_all(resolvers).await?
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

    pub fn collect_fields<'b, T: Resolver>(
        &'b self,
        parent_type: &'b T,
    ) -> Response<Vec<ResolverFuture<'b>>> {
        let mut resolvers: Vec<ResolverFuture<'b>> = Vec::new();
        for item in &self.item.items {
            match &item {
                Selection::Field(field) => {
                    if self.is_skip(&field.directives) {
                        continue;
                    }
                    if field.name == "__typename" {
                        let field_name = field.name.clone();

                        resolvers.push(Box::pin(async move {
                            Ok((field_name, GqlValue::String("typename".to_string())))
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
                                parent_type
                                    .resolve_field(&ctx_field)
                                    .await?
                                    .unwrap_or_default(),
                            ))
                        }
                    }))
                }
                Selection::FragmentSpread(fragment_spread) => {
                    let operation_fragment =
                        self.operation.fragments.get(&fragment_spread.fragment_name);
                    let fragment_def = match operation_fragment {
                        Some(fragment) => fragment,
                        None => {
                            return Err(GqlError::new(
                                format!("{:?} is not found in operation", fragment_spread),
                                Some(fragment_spread.position),
                            ))
                        }
                    };
                    self.with_selection_set(&fragment_def.selection_set)
                        .collect_fields(parent_type)?;
                }
                Selection::InlineFragment(inline_fragment) => {
                    if self.is_skip(&inline_fragment.directives) {
                        continue;
                    }
                    self.with_selection_set(&inline_fragment.selection_set)
                        .collect_fields(parent_type)?;
                }
            }
        }
        Ok(resolvers)
    }
}

pub(crate) fn build_context<'a>(
    schema: &'a ArcSchema,
    operation: &'a ArcOperation<'a>,
) -> ExecutionContext<'a, &'a Field<'a, String>> {
    let operation_type = operation.operation_type.to_string();
    let root_fieldname = operation.root_field.name.to_string();
    let current_field = &operation.root_field;

    let current_path = GraphQLPath::default()
        .prev(None)
        .current_key(root_fieldname)
        .parent_name(operation_type);

    ExecutionContext {
        schema,
        operation,
        item: current_field,
        current_path,
        errors: vec![],
    }
}
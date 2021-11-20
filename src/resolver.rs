use std::{
    collections::{BTreeMap, HashMap, HashSet},
    pin::Pin,
};

use async_trait::async_trait;
use futures::{future::BoxFuture, Future};
use graphql_parser::{
    query::{Field, Selection, SelectionSet},
    schema::Type,
};

use crate::{
    context::ExecutionContext, operation::Operation, path::GraphQLPath,
    types::value::value_from_ast, GqlType, GqlValue, Response, Schema,
};

// この型のvecを作成してfuture::joinに渡すことで並列に処理することができる。
// pub type ResolversFuture<'a> = BoxFuture<'a, Response<(String, GqlValue)>>;
pub type ResolversFuture = Pin<Box<dyn Future<Output = (String, GqlValue)> + Send + 'static>>;

#[async_trait]
pub trait Resolver {
    async fn resolve<'a>(
        &self,
        ctx: &ExecutionContext,
        field: &Field<'a, String>,
    ) -> Response<Option<GqlValue>>;
}

// pub(crate) struct ResolverInfo {
//     field_name: String,
//     return_type: GqlValue,
//     parent_type: String,
//     path: GraphQLPath,
// }

pub(crate) async fn resolve_query<'a, T: Resolver + ?Sized>(
    ctx: &ExecutionContext<'a>,
    root: &'a T,
) -> Response<GqlValue> {
    Ok(GqlValue::Null)
}

pub(crate) async fn resolve_mutation<'a, T: Resolver + ?Sized>(
    ctx: &ExecutionContext<'a>,
    root: &'a T,
) -> Response<GqlValue> {
    Ok(GqlValue::Null)
}

pub(crate) async fn resolve_subscription<'a, T: Resolver + ?Sized>(
    ctx: &ExecutionContext<'a>,
    root: &'a T,
) -> Response<GqlValue> {
    Ok(GqlValue::Null)
}

// TODO: schemaはfragmentの条件やskip directiveの処理で使用する
pub(crate) fn collect_query_fields<'a>(
    ctx: &'a ExecutionContext<'a>,
    selection_set: &'a SelectionSet<'a, String>,
) -> HashMap<String, Vec<Field<'a, String>>> {
    let mut fields: HashMap<String, Vec<Field<String>>> = HashMap::new();
    let mut visited_fragments = HashSet::new();

    collect_fields(&ctx, &selection_set, &mut fields, &mut visited_fragments);
    fields
}

fn collect_resolvers<'a, T: Resolver + 'a>(ctx: &'a ExecutionContext<'a>, root_resolver: &'a T) {
    let mut resolvers = Vec::new();
    for item in &ctx.operation.selection_set.items {
        match item {
            Selection::Field(field) => resolvers.push(Box::pin({
                if ctx.is_skip(&field.directives) {
                    continue;
                }

                async move {
                    let name = &field.name;
                    let ctx = ctx.clone();
                    (
                        name.clone(),
                        root_resolver
                            .resolve(&ctx, &field)
                            .await
                            .unwrap_or_default(),
                    )
                }
            })),
            Selection::FragmentSpread(fragment_spread) => {}
            Selection::InlineFragment(_) => todo!(),
        }
    }
}

fn collect_fields<'a>(
    ctx: &'a ExecutionContext<'a>,
    selection_set: &'a SelectionSet<'a, String>,
    fields: &mut HashMap<String, Vec<Field<'a, String>>>,
    visited_fragments: &mut HashSet<String>,
) {
    for item in &selection_set.items {
        match item {
            Selection::Field(field) => {
                if ctx.is_skip(&field.directives) {
                    continue;
                }

                if fields.contains_key(&field.name.to_string()) {
                    fields
                        .get_mut(&field.name.to_string())
                        .unwrap()
                        .push(field.clone());
                } else {
                    fields.insert(field.name.to_string(), vec![field.clone()]);
                }
            }
            Selection::FragmentSpread(spread_frg) => {
                let fragment_name = &spread_frg.fragment_name;
                if visited_fragments.contains(fragment_name) {
                    continue;
                }
                visited_fragments.insert(fragment_name.to_string());
                let fragment = &ctx.operation.fragments.get(fragment_name);
                match fragment {
                    Some(frg) => {
                        return collect_fields(&ctx, &frg.selection_set, fields, visited_fragments);
                    }
                    None => continue,
                }
            }
            Selection::InlineFragment(inline_frg) => {
                collect_fields(&ctx, &inline_frg.selection_set, fields, visited_fragments);
            }
        }
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

    use super::collect_query_fields;

    #[test]
    fn it_works() {
        let schema_doc = fs::read_to_string("src/tests/github.graphql").unwrap();
        let query_doc = fs::read_to_string("src/tests/github_query.graphql").unwrap();

        let schema = ArcSchema::new(build_schema(schema_doc.as_str()).unwrap());
        let query = build_operation(query_doc.as_str(), &schema, None).unwrap();

        let operation = ArcOperation::new(query);
        let context = build_context(&schema, &operation);

        let fields = collect_query_fields(&context, &operation.selection_set);

        for field in &fields {
            println!("{:?}", field);
        }

        for f in &fields["repository"] {
            for item in &f.selection_set.items {
                println!("{:?}", item);
            }
        }
    }
}

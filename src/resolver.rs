use std::{
    collections::{BTreeMap, HashMap},
    future,
};

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

// この型のvecを作成してfuture::joinに渡すことで並列に処理することができる。
type ResolverFuture<'a> = BoxFuture<'a, Response<(String, GqlValue)>>;
// pub type ResolverFuture<'a> = Pin<Box<dyn Future<Output = (String, Option<GqlValue>)> + Send + 'a>>;

// これを実装するのはqueryやgraphlqlのobject, Showなど
#[async_trait]
pub trait Resolver: Send + Sync {
    async fn resolve(&self, ctx: &ExecutionContext) -> Response<GqlValue>;
}

#[async_trait]
pub trait FieldResolver: Send + Sync {
    async fn resolve_field(&self, ctx: &ExecutionContext) -> Response<Option<GqlValue>>;
}

// pub(crate) struct ResolverInfo {
//     field_name: String,
//     return_type: GqlValue,
//     parent_type: String,
//     path: GraphQLPath,
// }

// ここの第２引数はqueryの起点となるrootのresolver
// それ以降はQueryで返されたstructのresolveを辿っていく
// impl Query
pub(crate) async fn resolve_query<'a, T: Resolver + ?Sized>(
    ctx: &ExecutionContext<'a>,
    query_resolvers: &'a T,
) -> Response<GqlValue> {
    Ok(GqlValue::Null)
}

pub(crate) async fn resolve_mutation<'a, T: Resolver + ?Sized>(
    ctx: &ExecutionContext<'a>,
    mutation_resolvers: &'a T,
) -> Response<GqlValue> {
    Ok(GqlValue::Null)
}

pub(crate) async fn resolve_subscription<'a, T: Resolver + ?Sized>(
    ctx: &ExecutionContext<'a>,
    subscription_resolvers: &'a T,
) -> Response<GqlValue> {
    Ok(GqlValue::Null)
}

fn build_gql_object(target_map: &mut BTreeMap<String, GqlValue>, gql_value: (String, GqlValue)) {
    let (field_name, value) = gql_value;
    if let Some(prev_value) = target_map.get_mut(&field_name) {
        match prev_value {
            GqlValue::List(target_list) => if let GqlValue::List(list) = value {},
            GqlValue::Object(target_obj) => {
                if let GqlValue::Object(obj) = value {
                    for map in obj.into_iter() {
                        build_gql_object(target_map, (map.0, *map.1))
                    }
                }
            }
            _ => return,
        }
    } else {
        target_map.insert(field_name, value.clone());
    }
}

pub struct Resolvers<'a>(Vec<ResolverFuture<'a>>);

pub async fn resolve_object<'a, T: Resolver>(
    ctx: &'a ExecutionContext<'a>,
    parent_type: &'a T,
    parallel: bool,
) -> Response<GqlValue> {
    let mut resolvers = Resolvers(Vec::new());

    resolvers.collect_field_resolvers(ctx, parent_type, &ctx.operation.selection_set)?;

    let res = if parallel {
        futures::future::try_join_all(resolvers.0).await?
    } else {
        let mut results = Vec::new();
        for resolver in resolvers.0 {
            results.push(resolver.await?);
        }
        results
    };

    let mut target_map = BTreeMap::new();

    for value in res {
        build_gql_object(&target_map, res);
    };

    Ok(GqlValue::Null)
}

impl<'a> Resolvers<'a> {
    pub fn collect_field_resolvers<T: Resolver + 'a>(
        &mut self,
        ctx: &'a ExecutionContext<'a>,
        parent_type: &'a T,
        selection_set: &'a SelectionSet<'a, String>,
    ) -> Response<()> {
        for item in &selection_set.items {
            match item {
                Selection::Field(field) => {
                    if ctx.is_skip(&field.directives) {
                        continue;
                    }
                    self.0.push(Box::pin({
                        async move {
                            let mut ctx = ctx.clone();
                            ctx.current_field(field.clone());
                            let field_name = &field.name;
                            Ok((
                                field_name.clone(),
                                parent_type.resolve(&ctx).await.unwrap_or_default(),
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
                    self.collect_field_resolvers(ctx, parent_type, &fragment_def.selection_set)?;
                }
                Selection::InlineFragment(inline_fragment) => {
                    if ctx.is_skip(&inline_fragment.directives) {
                        continue;
                    }
                    self.collect_field_resolvers(ctx, parent_type, &inline_fragment.selection_set)?;
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

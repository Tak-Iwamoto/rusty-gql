use std::collections::HashMap;

use async_trait::async_trait;
use futures::future::BoxFuture;
use graphql_parser::{query::Field, schema::Type};

use crate::{
    context::{FieldContext, SelectionSetContext},
    GqlTypeDefinition, GqlValue, Response, Schema,
};

pub type ResolverFuture<'a> = BoxFuture<'a, Response<(String, GqlValue)>>;

#[async_trait]
pub trait SelectionSetResolver: Resolver {
    async fn resolve_selection_set(&self, ctx: &SelectionSetContext<'_>) -> Response<GqlValue>;
}

#[async_trait]
pub trait Resolver: Send + Sync {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> Response<Option<GqlValue>>;
}

#[async_trait::async_trait]
impl<T: Resolver> Resolver for &T {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> Response<Option<GqlValue>> {
        T::resolve_field(*self, ctx).await
    }
}

// pub fn get_variable_values<'a>(
//     schema: &'a Schema,
//     operation: &'a Operation<'a>,
//     input_values: &BTreeMap<String, GqlValue>,
// ) -> Result<HashMap<String, GqlValue>, String> {
//     let mut variables = HashMap::new();
//     for var in &operation.variable_definitions {
//         let var_type = get_type_from_schema(schema, &var.var_type);

//         let var_type = match var_type {
//             Some(ty) => ty,
//             None => continue,
//         };

//         let var_name = &var.name.to_string();
//         if !input_values.contains_key(var_name) {
//             if let Some(value) = &var.default_value {
//                 variables.insert(
//                     var.name.to_string(),
//                     value_from_ast(value, &var_type, &None),
//                 );
//             }
//         }

//         let value = input_values.get(var_name);

//         if let GqlMetaType::NonNull(_) = var_type {
//             if value.is_none() {
//                 return Err(format!("{} must not be null", var_name));
//             }
//         }

//         if let Some(var_value) = value {
//             variables.insert(var_name.to_string(), var_value.clone());
//         }
//     }
//     Ok(variables)
// }

pub fn get_arguments<'a>(field: Field<'a, String>, variable_values: HashMap<String, GqlValue>) {
    let arguments = field.arguments;
}

pub fn get_type_from_schema<'a>(
    schema: &'a Schema,
    var_type: &'a Type<'a, String>,
) -> Option<GqlTypeDefinition> {
    match var_type {
        graphql_parser::schema::Type::NamedType(named_type) => schema
            .type_definitions
            .get(&named_type.to_string())
            .map(|var_ty| var_ty.clone()),
        graphql_parser::schema::Type::ListType(list) => {
            let inner_type = get_type_from_schema(schema, &list).unwrap();
            Some(inner_type)
        }
        graphql_parser::schema::Type::NonNullType(non_null) => {
            let inner_type = get_type_from_schema(schema, &non_null).unwrap();
            Some(inner_type)
        }
    }
}

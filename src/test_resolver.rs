use std::fs;

use crate::{
    context::{collect_all_fields, ExecutionContext},
    operation::build_operation,
    path::GraphQLPath,
    types::schema::{build_schema, ArcSchema},
};

fn test_query() {
    let contents = fs::read_to_string("src/tests/github.graphql").unwrap();
    let schema = build_schema(contents.as_str()).unwrap();
    let query_doc = fs::read_to_string("src/tests/pet_query.graphql").unwrap();
    let query = build_operation(query_doc.as_str(), &ArcSchema::new(schema), None).unwrap();
    let root_fields = collect_all_fields(&schema, &query, &query.definition.selection_set);

    for (response_name, fields) in &root_fields {
        let gql_path = GraphQLPath {
            prev: None,
            key: response_name.to_string(),
            // TODO: operationのtypeごとに変える
            parent_name: "query".to_string(),
        };
    }
    println!("{:?}", root_fields.keys().len());
    println!("{:?}", root_fields);
}

// fn execute_field<'a>(
//     ctx: &ExecutionContext,
//     parent_type: &GraphQLObject,
//     fields: &Vec<Field<'a, String>>,
//     path: &GraphQLPath,
//     is_root: bool,
// ) {
//     let field_name = fields.first().unwrap().name;
//     let root_def = match parent_type.name.as_str() {
//         "Query" => ctx.schema.queries.get(&field_name.to_string()).unwrap(),
//         "Mutation" => ctx.schema.mutations.get(&field_name.to_string()).unwrap(),
//         "Subscription" => ctx
//             .schema
//             .subscriptions
//             .get(&field_name.to_string())
//             .unwrap(),
//         _ => unreachable!(),
//     };

//     let return_type = &root_def.field_type;
//     match return_type {
//         graphql_parser::schema::Type::NamedType(_) => todo!(),
//         graphql_parser::schema::Type::ListType(_) => todo!(),
//         graphql_parser::schema::Type::NonNullType(_) => todo!(),
//     }
//     let resolve_fn = todo!();

//     let args = todo!();

//     // let resolver_info = build_resolver_info();

//     // let result = resolve_fn(&args, ctx.context_value, )
// }

#[cfg(test)]
mod tests {
    use super::test_query;

    #[test]
    fn it_works() {
        test_query()
    }
}

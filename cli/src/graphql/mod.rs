use std::collections::BTreeMap;

use codegen::Scope;
use futures::future::try_join_all;
use rusty_gql::{self, build_schema, GqlField, GqlType};

fn read_graphql_schema(schema_doc: &str) -> Result<(), String> {
    let schema = build_schema(schema_doc)?;

    let types = schema.type_map;
    let queries = schema.queries;
    // let mut scope = Scope::new();
    generate_operations(&queries);
    Ok(())
}

async fn generate_operations<'a>(operations: &BTreeMap<String, GqlField>) {
    let mut futures = Vec::new();
    for (key, field) in operations.iter() {
        let create_file_fn = tokio::fs::File::create(format!("{:?}.rs", field.name));
        futures.push(create_file_fn);
    }
    let res = try_join_all(futures).await;
}

fn gerate_types(types_map: &BTreeMap<String, GqlType>) {
    for (key, gql_type) in types_map.iter() {
        let mut scope = Scope::new();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use rusty_gql::build_schema;

    use super::generate_operations;

    #[tokio::test]
    async fn it_works() {
        let schema_doc = fs::read_to_string("../src/tests/github.graphql").unwrap();
        let schema = build_schema(schema_doc.as_str()).unwrap();
        generate_operations(&schema.queries).await;
    }
}

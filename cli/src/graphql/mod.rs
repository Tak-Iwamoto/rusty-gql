use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures::{future::try_join_all, TryFutureExt};
use rusty_gql::{self, build_schema, GqlField, GqlType};
use tokio::{fs::File, io::AsyncWriteExt};

fn read_graphql_schema(schema_doc: &str) -> Result<(), String> {
    let schema = build_schema(schema_doc)?;

    let types = schema.type_map;
    let queries = schema.queries;
    // let mut scope = Scope::new();
    generate_operations(&queries);
    Ok(())
}

async fn generate_operations<'a>(
    operations: &BTreeMap<String, GqlField>,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    for (_, field) in operations.iter() {
        let task = generate_operation_file(field);
        futures.push(task);
    }
    let res = try_join_all(futures).await;
    res
}

async fn create_dirs() -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    // dirを作るときはcliのroot配下に作成される
    futures.push(tokio::fs::create_dir_all("./queries"));
    futures.push(tokio::fs::create_dir_all("./mutations"));
    futures.push(tokio::fs::create_dir_all("./subscriptions"));
    futures.push(tokio::fs::create_dir_all("./models"));
    let res = try_join_all(futures).await;
    res
}

fn generate_field_str(field: &GqlField) -> String {
    let mut scope = Scope::new();
    let fn_scope = scope.new_fn(field.name.as_str());

    for arg in &field.arguments {
        fn_scope.arg(arg.name.as_str(), "String");
    }
    scope.to_string()
}

async fn generate_operation_file(field: &GqlField) -> Result<(), Error> {
    let mut file = tokio::fs::File::create(format!("queries/{}.rs", field.name)).await?;
    file.write(generate_field_str(&field).as_bytes()).await?;
    Ok(())
}

fn generate_types(types_map: &BTreeMap<String, GqlType>) {
    for (key, gql_type) in types_map.iter() {
        let mut scope = Scope::new();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use rusty_gql::build_schema;

    use crate::graphql::create_dirs;

    use super::generate_operations;

    #[tokio::test]
    async fn it_works() {
        let schema_doc = fs::read_to_string("../src/tests/github.graphql").unwrap();
        let schema = build_schema(schema_doc.as_str()).unwrap();
        create_dirs().await;
        let res = generate_operations(&schema.queries).await;
        println!("{:?}", res)
        // let field = schema.queries.get("repository").unwrap();
        // let result = generate_field_str(field);
        // println!("{}", result);
    }
}

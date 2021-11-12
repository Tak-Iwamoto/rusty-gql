use std::collections::BTreeMap;

use codegen::Scope;
use rusty_gql::{self, build_schema, GraphQLType};

fn read_graphql_schema(schema_doc: &str) -> Result<(), String> {
    let schema = build_schema(schema_doc)?;

    let types = schema.type_map;
    let queries = schema.queries;
    // let mut scope = Scope::new();
    // generate_operations(&queries);
    Ok(())
}

fn generate_operations<'a>(
    operations: &BTreeMap<String, graphql_parser::schema::Field<'a, String>>,
) {
    for (key, field) in operations.iter() {
        let mut scope = Scope::new();
    }
}

fn gerate_types(types_map: &BTreeMap<String, GraphQLType>) {
    for (key, gql_type) in types_map.iter() {
        let mut scope = Scope::new();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use rusty_gql::{build_schema, GraphQLType};

    #[test]
    fn it_works() {
        let schema_doc = fs::read_to_string("../src/tests/github.graphql").unwrap();
        let schema = build_schema(schema_doc.as_str()).unwrap();
        let target = schema.type_map.get("App").unwrap();

        if let GraphQLType::Object(obj) = target {
            println!("{:?}", &obj.fields);
        }
        // for (key, gql_type) in schema.type_map.iter() {
        //     println!("{:?}", key);
        //     println!("{:?}", gql_type);
        // }
    }
}

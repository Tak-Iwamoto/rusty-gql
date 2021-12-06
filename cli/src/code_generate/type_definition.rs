use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures::future::try_join_all;
use rusty_gql::GqlTypeDefinition;

use super::utils::{create_file, PathStr};

pub async fn gen_type_definition_files(
    type_definitions: &BTreeMap<String, GqlTypeDefinition>,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    for (_, type_def) in type_definitions.iter() {
        let task = gen_type_definition_file(type_def);
        futures.push(task);
    }
    let res = try_join_all(futures).await;
    res
}

async fn gen_type_definition_file(type_def: &GqlTypeDefinition) -> Result<(), Error> {
    let path =
        PathStr::new(vec![&type_def.to_string().to_lowercase(), type_def.name()]).to_string();
    if tokio::fs::File::open(&path).await.is_err() {
        let content = gen_type_definition_str(&type_def);
        create_file(&path, &content).await?;
        Ok(())
    } else {
        Ok(())
    }
}

fn gen_type_definition_str(type_def: &GqlTypeDefinition) -> String {
    let mut scope = Scope::new();
    let struct_scope = scope.new_struct(type_def.name());

    if let Some(fields) = type_def.fields() {
        for field in fields {
            struct_scope.field(&field.name, field.meta_type.name());
        }
    }

    scope.to_string()
}

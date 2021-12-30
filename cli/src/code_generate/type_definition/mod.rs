mod enum_file;
mod input_file;
mod interface_file;
mod object_file;
mod scalar_file;
mod union_file;

use futures_util::future::try_join_all;
use rusty_gql::GqlTypeDefinition;
use std::{collections::BTreeMap, io::Error};

use self::{
    enum_file::EnumFile, input_file::InputObjectFile, interface_file::InterfaceFile,
    object_file::ObjectFile, scalar_file::ScalarFile, union_file::UnionFile,
};

use super::utils::{create_file, PathStr};

pub trait TypeDefinitionFileStrategy {
    fn content(&self) -> String;

    fn file_name(&self) -> String;

    fn base_path(&self) -> String;
}

pub async fn build_type_definition_files(
    type_definitions: &BTreeMap<String, GqlTypeDefinition>,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    for (_, type_def) in type_definitions.iter() {
        if reserved_scalar_names().contains(&type_def.name()) {
            continue;
        }
        let task = build_type_definition_file(type_def);
        futures.push(task);
    }
    let res = try_join_all(futures).await;
    res
}

fn reserved_scalar_names() -> Vec<&'static str> {
    vec!["String", "Int", "Float", "Boolean"]
}

async fn build_type_definition_file(type_def: &GqlTypeDefinition) -> Result<(), Error> {
    match type_def {
        GqlTypeDefinition::Scalar(def) => build_type_def_file(ScalarFile { def }).await,
        GqlTypeDefinition::Object(def) => build_type_def_file(ObjectFile { def }).await,
        GqlTypeDefinition::Interface(def) => build_type_def_file(InterfaceFile { def }).await,
        GqlTypeDefinition::Union(def) => build_type_def_file(UnionFile { def }).await,
        GqlTypeDefinition::Enum(def) => build_type_def_file(EnumFile { def }).await,
        GqlTypeDefinition::InputObject(def) => build_type_def_file(InputObjectFile { def }).await,
    }
}

async fn build_type_def_file<T: TypeDefinitionFileStrategy>(strategy: T) -> Result<(), Error> {
    let base_path = strategy.base_path();
    let file_name = strategy.file_name();
    let path = PathStr {
        paths: vec![base_path.as_str(), file_name.as_str()],
        base_path: None,
    }
    .to_string();
    if tokio::fs::File::open(&path).await.is_err() {
        let content = strategy.content();
        create_file(&path, &content).await?;
        Ok(())
    } else {
        Ok(())
    }
}

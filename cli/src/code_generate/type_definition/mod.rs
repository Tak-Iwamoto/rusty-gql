mod enum_file;
mod input_file;
mod interface_file;
mod object_file;
mod scalar_file;
mod union_file;

use futures_util::future::try_join_all;
use heck::ToSnakeCase;
use rusty_gql::{GqlTypeDefinition, Schema};
use std::io::Error;

use self::{
    enum_file::EnumFile, input_file::InputObjectFile, interface_file::InterfaceFile,
    object_file::ObjectFile, scalar_file::ScalarFile, union_file::UnionFile,
};

use super::{create_file, dir_path_str, file_path_str, mod_file::ModFile};

pub async fn create_type_definition_files(
    schema: &Schema,
    base_path: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut model_file_names = Vec::new();
    let mut interface_file_names = Vec::new();
    let mut input_file_names = Vec::new();
    let mut scalar_file_names = Vec::new();

    for (_, type_def) in schema.type_definitions.iter() {
        if reserved_scalar_names().contains(&type_def.name()) {
            continue;
        }
        let operation_type_names = vec![
            &schema.query_type_name,
            &schema.mutation_type_name,
            &schema.subscription_type_name,
        ];
        let name = type_def.name();
        if operation_type_names.contains(&&name.to_string()) {
            continue;
        }

        match type_def {
            GqlTypeDefinition::Union(v) => model_file_names.push(v.name.clone()),
            GqlTypeDefinition::Enum(v) => model_file_names.push(v.name.clone()),
            GqlTypeDefinition::Object(v) => model_file_names.push(v.name.clone()),
            GqlTypeDefinition::Interface(v) => interface_file_names.push(v.name.clone()),
            GqlTypeDefinition::InputObject(v) => input_file_names.push(v.name.clone()),
            GqlTypeDefinition::Scalar(v) => scalar_file_names.push(v.name.clone()),
        }

        let task = create_type_definition_file(type_def, base_path, interface_file_names.clone());
        futures.push(task);
    }

    create_file(ModFile {
        path: &dir_path_str(vec![base_path, "model"]),
        file_names: model_file_names,
    })
    .await?;

    create_file(ModFile {
        path: &dir_path_str(vec![base_path, "interface"]),
        file_names: interface_file_names,
    })
    .await?;

    create_file(ModFile {
        path: &dir_path_str(vec![base_path, "input"]),
        file_names: input_file_names,
    })
    .await?;

    create_file(ModFile {
        path: &dir_path_str(vec![base_path, "scalar"]),
        file_names: scalar_file_names,
    })
    .await?;

    try_join_all(futures).await
}

fn reserved_scalar_names() -> Vec<&'static str> {
    vec!["String", "Int", "Float", "Boolean", "ID"]
}

async fn create_type_definition_file(
    type_def: &GqlTypeDefinition,
    base_path: &str,
    interface_names: Vec<String>,
) -> Result<(), Error> {
    match type_def {
        GqlTypeDefinition::Scalar(def) => {
            let path = file_path_str(vec![base_path, "scalar", &type_def.name().to_snake_case()]);
            create_file(ScalarFile { def, path: &path }).await
        }
        GqlTypeDefinition::Object(def) => {
            let path = file_path_str(vec![base_path, "model", &type_def.name().to_snake_case()]);
            create_file(ObjectFile {
                def,
                path: &path,
                interface_names: &interface_names,
            })
            .await
        }
        GqlTypeDefinition::Interface(def) => {
            let path = file_path_str(vec![
                base_path,
                "interface",
                &type_def.name().to_snake_case(),
            ]);
            create_file(InterfaceFile {
                def,
                path: &path,
                interface_names: &interface_names,
            })
            .await
        }
        GqlTypeDefinition::Union(def) => {
            let path = file_path_str(vec![base_path, "model", &type_def.name().to_snake_case()]);
            create_file(UnionFile { def, path: &path }).await
        }
        GqlTypeDefinition::Enum(def) => {
            let path = file_path_str(vec![base_path, "model", &type_def.name().to_snake_case()]);
            create_file(EnumFile { def, path: &path }).await
        }
        GqlTypeDefinition::InputObject(def) => {
            let path = file_path_str(vec![base_path, "input", &type_def.name().to_snake_case()]);
            create_file(InputObjectFile { def, path: &path }).await
        }
    }
}

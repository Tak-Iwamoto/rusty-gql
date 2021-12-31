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

use super::{build_dir_path_str, build_file_path_str, create_file, mod_file::ModFile};

pub async fn create_type_definition_files(
    type_definitions: &BTreeMap<String, GqlTypeDefinition>,
    base_path: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut model_file_names = Vec::new();
    let mut interface_file_names = Vec::new();
    let mut input_file_names = Vec::new();
    let mut scalar_file_names = Vec::new();

    for (_, type_def) in type_definitions.iter() {
        if reserved_scalar_names().contains(&type_def.name()) {
            continue;
        }
        let task = create_type_definition_file(type_def, base_path);
        futures.push(task);

        match type_def {
            GqlTypeDefinition::Union(v) => model_file_names.push(v.name.clone()),
            GqlTypeDefinition::Enum(v) => model_file_names.push(v.name.clone()),
            GqlTypeDefinition::Object(v) => model_file_names.push(v.name.clone()),
            GqlTypeDefinition::Interface(v) => interface_file_names.push(v.name.clone()),
            GqlTypeDefinition::InputObject(v) => input_file_names.push(v.name.clone()),
            GqlTypeDefinition::Scalar(v) => scalar_file_names.push(v.name.clone()),
        }
    }

    create_file(ModFile {
        path: &build_dir_path_str(base_path, vec!["model"]),
        file_names: model_file_names,
    })
    .await?;

    create_file(ModFile {
        path: &build_dir_path_str(base_path, vec!["interface"]),
        file_names: interface_file_names,
    })
    .await?;

    create_file(ModFile {
        path: &build_dir_path_str(base_path, vec!["input"]),
        file_names: input_file_names,
    })
    .await?;

    create_file(ModFile {
        path: &build_dir_path_str(base_path, vec!["scalar"]),
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
) -> Result<(), Error> {
    let model_file_path = build_file_path_str(base_path, vec!["model", &type_def.name()]);
    let scalar_file_path = build_file_path_str(base_path, vec!["scalar", &type_def.name()]);
    let interface_file_path = build_file_path_str(base_path, vec!["interface", &type_def.name()]);
    let input_file_path = build_file_path_str(base_path, vec!["input", &type_def.name()]);
    match type_def {
        GqlTypeDefinition::Scalar(def) => {
            create_file(ScalarFile {
                def,
                path: &scalar_file_path,
            })
            .await
        }
        GqlTypeDefinition::Object(def) => {
            create_file(ObjectFile {
                def,
                path: &model_file_path,
            })
            .await
        }
        GqlTypeDefinition::Interface(def) => {
            create_file(InterfaceFile {
                def,
                path: &interface_file_path,
            })
            .await
        }
        GqlTypeDefinition::Union(def) => {
            create_file(UnionFile {
                def,
                path: &model_file_path,
            })
            .await
        }
        GqlTypeDefinition::Enum(def) => {
            create_file(EnumFile {
                def,
                path: &model_file_path,
            })
            .await
        }
        GqlTypeDefinition::InputObject(def) => {
            create_file(InputObjectFile {
                def,
                path: &input_file_path,
            })
            .await
        }
    }
}

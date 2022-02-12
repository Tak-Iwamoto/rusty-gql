mod enum_file;
mod input_file;
mod interface_file;
mod object_file;
mod scalar_file;
mod union_file;

use futures_util::future::try_join_all;
use heck::ToSnakeCase;
use rusty_gql::{Schema, TypeDefinition};
use std::{collections::HashMap, io::Error};

use self::{
    enum_file::EnumFile, input_file::InputObjectFile, interface_file::InterfaceFile,
    object_file::ObjectFile, scalar_file::ScalarFile, union_file::UnionFile,
};

use super::{
    create_file,
    mod_file::ModFile,
    path_str,
    util::{is_gql_primitive_ty, is_introspection_type_names},
};

pub async fn create_type_definition_files(
    schema: &Schema,
    base_path: &str,
    interface_obj_map: &HashMap<String, Vec<String>>,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut resolver_names = Vec::new();
    let mut input_names = Vec::new();
    let mut scalar_names = Vec::new();

    for (_, type_def) in schema.type_definitions.iter() {
        if is_gql_primitive_ty(type_def.name()) {
            continue;
        }
        if is_introspection_type_names(type_def.name()) {
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
            TypeDefinition::Union(v) => resolver_names.push(v.name.clone()),
            TypeDefinition::Enum(v) => resolver_names.push(v.name.clone()),
            TypeDefinition::Object(v) => resolver_names.push(v.name.clone()),
            TypeDefinition::Interface(v) => resolver_names.push(v.name.clone()),
            TypeDefinition::InputObject(v) => input_names.push(v.name.clone()),
            TypeDefinition::Scalar(v) => scalar_names.push(v.name.clone()),
        }

        let task = create_type_definition_file(type_def, base_path, interface_obj_map);
        futures.push(task);
    }

    create_file(ModFile {
        path: &path_str(vec![base_path, "resolver"], false),
        struct_names: resolver_names,
    })
    .await?;

    create_file(ModFile {
        path: &path_str(vec![base_path, "input"], false),
        struct_names: input_names,
    })
    .await?;

    create_file(ModFile {
        path: &path_str(vec![base_path, "scalar"], false),
        struct_names: scalar_names,
    })
    .await?;

    try_join_all(futures).await
}

async fn create_type_definition_file(
    type_def: &TypeDefinition,
    base_path: &str,
    interface_obj_map: &HashMap<String, Vec<String>>,
) -> Result<(), Error> {
    let filename = type_def.name().to_snake_case();
    match type_def {
        TypeDefinition::Object(def) => {
            let path = path_str(vec![base_path, "resolver", &filename], true);
            ObjectFile {
                def,
                path: &path,
                filename: &filename,
            }
            .create_file()
            .await
        }
        TypeDefinition::Interface(def) => {
            let path = path_str(vec![base_path, "resolver", &filename], true);
            InterfaceFile {
                def,
                path: &path,
                filename: &filename,
                interface_obj_map,
            }
            .create_file()
            .await
        }
        TypeDefinition::Union(def) => {
            let path = path_str(vec![base_path, "resolver", &filename], true);
            UnionFile {
                def,
                path: &path,
                filename: &filename,
            }
            .create_file()
            .await
        }
        TypeDefinition::Enum(def) => {
            let path = path_str(vec![base_path, "resolver", &filename], true);
            EnumFile {
                def,
                path: &path,
                filename: &filename,
            }
            .create_file()
            .await
        }
        TypeDefinition::InputObject(def) => {
            let path = path_str(vec![base_path, "input", &filename], true);
            InputObjectFile {
                def,
                path: &path,
                filename: &filename,
            }
            .create_file()
            .await
        }
        TypeDefinition::Scalar(def) => {
            let path = path_str(vec![base_path, "scalar", &filename], true);
            create_file(ScalarFile {
                def,
                path: &path,
                filename: &filename,
            })
            .await
        }
    }
}

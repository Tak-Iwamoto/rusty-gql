use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures_util::future::try_join_all;
use heck::{ToPascalCase, ToSnakeCase};
use rusty_gql::GqlDirectiveDefinition;

use crate::code_generate::FileDefinition;

use super::{create_file, mod_file::ModFile, path_str, util::gql_value_ty_to_rust_ty};

pub struct DirectiveFile<'a> {
    pub def: &'a GqlDirectiveDefinition,
    pub path: String,
    pub file_name: String,
}

impl<'a> FileDefinition for DirectiveFile<'a> {
    fn content(&self) -> String {
        let mut scope = Scope::new();
        let struct_scope = scope.new_struct(&self.def.name.to_pascal_case()).vis("pub");

        for field in &self.def.arguments {
            struct_scope.field(
                &field.name.to_snake_case(),
                gql_value_ty_to_rust_ty(&field.meta_type),
            );
        }

        scope.to_string()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn name(&self) -> String {
        self.file_name.clone()
    }
}

pub async fn create_directive_files(
    directives: &BTreeMap<String, GqlDirectiveDefinition>,
    base_path: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut directive_names = Vec::new();
    for (_, directive) in directives.iter() {
        let dir_file_name = &directive.name.to_snake_case();
        let path = path_str(vec![base_path, "directive", &dir_file_name], true);
        futures.push(create_file(DirectiveFile {
            def: directive,
            path,
            file_name: dir_file_name.clone(),
        }));
        directive_names.push(directive.name.to_pascal_case().clone());
    }
    create_file(ModFile {
        path: &path_str(vec![base_path, "directive"], false),
        struct_names: directive_names,
    })
    .await?;

    try_join_all(futures).await
}

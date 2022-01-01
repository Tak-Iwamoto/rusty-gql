use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures_util::future::try_join_all;
use rusty_gql::GqlDirectiveDefinition;

use crate::code_generate::FileDefinition;

use super::{
    create_file, dir_path_str, file_path_str, mod_file::ModFile, util::gql_value_ty_to_rust_ty,
};

pub struct DirectiveFile<'a> {
    pub def: &'a GqlDirectiveDefinition,
    pub path: String,
}

impl<'a> FileDefinition for DirectiveFile<'a> {
    fn content(&self) -> String {
        let mut scope = Scope::new();
        let struct_scope = scope.new_struct(&self.def.name).vis("pub");

        for field in &self.def.arguments {
            struct_scope.field(&field.name, gql_value_ty_to_rust_ty(&field.meta_type));
        }

        scope.to_string()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }
}

pub async fn create_directive_files(
    directives: &BTreeMap<String, GqlDirectiveDefinition>,
    base_path: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut file_names = Vec::new();
    for (_, directive) in directives.iter() {
        let path = file_path_str(vec![base_path, "directive", &directive.name]);
        futures.push(create_file(DirectiveFile {
            def: directive,
            path,
        }));
        file_names.push(directive.name.clone());
    }
    create_file(ModFile {
        path: &dir_path_str(vec![base_path, "directive"]),
        file_names,
    })
    .await?;

    try_join_all(futures).await
}

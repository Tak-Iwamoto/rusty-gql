use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures_util::future::try_join_all;
use rusty_gql::GqlDirectiveDefinition;

use crate::code_generate::FileStrategy;

use super::{build_dir_path_str, build_file_path_str, create_file, graphql_mod_file::GqlModFile};

pub struct DirectiveFile<'a> {
    pub def: &'a GqlDirectiveDefinition,
    pub base_path: String,
}

impl<'a> FileStrategy for DirectiveFile<'a> {
    fn content(&self) -> String {
        let mut scope = Scope::new();
        let struct_scope = scope.new_struct(self.def.name.as_str()).vis("pub");

        for field in &self.def.arguments {
            struct_scope.field(&field.name, field.meta_type.to_rust_type_str());
        }

        scope.to_string()
    }

    fn path(&self) -> String {
        build_file_path_str(&self.base_path, vec!["directive", &self.def.name])
    }
}

pub async fn create_directive_files(
    directives: &BTreeMap<String, GqlDirectiveDefinition>,
    base_path: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut file_names = Vec::new();
    for (_, directive) in directives.iter() {
        futures.push(create_file(DirectiveFile {
            def: directive,
            base_path: base_path.to_string(),
        }));
        file_names.push(directive.name.clone());
    }
    create_file(GqlModFile {
        path: &build_dir_path_str(base_path, vec!["directive"]),
        file_names,
    })
    .await?;

    try_join_all(futures).await
}

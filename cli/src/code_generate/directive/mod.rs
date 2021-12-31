use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures_util::future::try_join_all;
use rusty_gql::GqlDirectiveDefinition;

use crate::code_generate::FileStrategy;

use super::{build_file, concat_file_path, graphql_mod_file::GqlModFile};

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
        concat_file_path(&self.base_path, vec!["directive", &self.def.name])
    }
}

pub async fn create_directive_files(
    directives: &BTreeMap<String, GqlDirectiveDefinition>,
    base_path: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut file_names = Vec::new();
    for (_, directive) in directives.iter() {
        futures.push(build_file(DirectiveFile {
            def: directive,
            base_path: base_path.to_string(),
        }));
        file_names.push(directive.name.clone());
    }
    build_file(GqlModFile {
        path: &format!("{}/{}", base_path, "directive"),
        file_names,
    })
    .await?;

    try_join_all(futures).await
}

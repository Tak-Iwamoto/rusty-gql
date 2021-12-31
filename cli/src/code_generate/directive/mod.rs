use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures_util::future::try_join_all;
use rusty_gql::GqlDirectiveDefinition;

use crate::code_generate::FileStrategy;

use super::{build_file, graphql_file_path, graphql_mod_file::ModFile};

pub struct DirectiveFile<'a> {
    pub def: &'a GqlDirectiveDefinition,
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
        graphql_file_path(vec!["directive", &self.def.name])
    }
}

pub async fn create_directive_files(
    directives: &BTreeMap<String, GqlDirectiveDefinition>,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut file_names = Vec::new();
    for (_, directive) in directives.iter() {
        futures.push(build_file(DirectiveFile { def: directive }));
        file_names.push(directive.name.clone());
    }
    build_file(ModFile {
        base_path: "directive".to_string(),
        file_names,
    })
    .await?;

    try_join_all(futures).await
}

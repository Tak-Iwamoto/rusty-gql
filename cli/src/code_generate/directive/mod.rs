use std::{collections::HashMap, io::Error};

use codegen::Scope;
use futures_util::future::try_join_all;
use rusty_gql::GqlDirectiveDefinition;

use crate::code_generate::FileDefinition;

use super::{create_file, mod_file::ModFile, path_str};

pub struct DirectiveFile<'a> {
    pub def: &'a GqlDirectiveDefinition,
    pub path: String,
    pub file_name: String,
}

impl<'a> FileDefinition for DirectiveFile<'a> {
    fn content(&self) -> String {
        let mut scope = Scope::new();
        scope.new_struct(&self.def.name).vis("pub");
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
    directives: &HashMap<String, GqlDirectiveDefinition>,
    base_path: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut directive_names = Vec::new();
    for (_, directive) in directives.iter() {
        if is_default_directive(&directive.name) {
            continue;
        }
        let file_name = &directive.name;
        let path = path_str(vec![base_path, "directive", &file_name], true);
        futures.push(create_file(DirectiveFile {
            def: directive,
            path,
            file_name: file_name.clone(),
        }));
        directive_names.push(directive.name.clone());
    }
    create_file(ModFile {
        path: &path_str(vec![base_path, "directive"], false),
        struct_names: directive_names,
    })
    .await?;

    try_join_all(futures).await
}

fn is_default_directive(name: &str) -> bool {
    vec!["skip", "include", "deprecated"].contains(&name)
}

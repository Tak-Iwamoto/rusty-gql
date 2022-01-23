use std::{collections::HashMap, io::Error};

use codegen::Scope;
use futures_util::future::try_join_all;
use heck::ToSnakeCase;
use rusty_gql::GqlDirectiveDefinition;

use crate::code_generate::{use_gql_definitions, FileDefinition};

use super::{create_file, mod_file::ModFile, path_str};

pub struct DirectiveFile<'a> {
    pub def: &'a GqlDirectiveDefinition,
    pub path: String,
    pub file_name: String,
}

impl<'a> FileDefinition for DirectiveFile<'a> {
    fn content(&self) -> String {
        let mut scope = Scope::new();
        let struct_name = &self.def.name;
        scope.new_struct(struct_name).vis("pub");
        let directive_impl = scope.new_impl(struct_name);
        directive_impl.impl_trait("CustomDirective");
        directive_impl.r#macro("#[async_trait::async_trait]");

        let f = directive_impl.new_fn("resolve_field");
        f.set_async(true);
        f.arg_ref_self();
        f.arg("ctx", "&FieldContext<'_>");
        f.arg("directive_args", "&BTreeMap<String, GqlValue>");
        f.arg("resolve_fut", "ResolveFut<'_>");
        f.ret("ResolverResult<Option<GqlValue>>");
        f.line("todo!()");

        format!(
            "{}\nuse std::collections::BTreeMap;\n\n{}",
            use_gql_definitions(),
            scope.to_string()
        )
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
        let file_name = &directive.name.to_snake_case();
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

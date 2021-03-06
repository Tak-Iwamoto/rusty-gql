use std::{collections::HashMap, io::Error};

use codegen::Scope;
use futures_util::future::try_join_all;
use heck::{ToSnakeCase, ToUpperCamelCase};
use rusty_gql::DirectiveDefinition;

use crate::code_generate::{use_gql_definitions, FileDefinition};

use super::{create_file, mod_file::ModFile, path_str, CreateFile};

pub struct DirectiveFile<'a> {
    pub def: &'a DirectiveDefinition,
    pub path: String,
    pub filename: String,
}

impl<'a> FileDefinition for DirectiveFile<'a> {
    fn content(&self) -> String {
        let mut scope = Scope::new();
        let struct_name = &self.def.name.to_upper_camel_case();
        scope.new_struct(struct_name).vis("pub");
        let new_impl = scope.new_impl(struct_name);
        let new_fn = new_impl.new_fn("new");
        new_fn.ret("Box<dyn CustomDirective>");
        new_fn.line(format!("Box::new({} {{}})", struct_name));

        let directive_impl = scope.new_impl(struct_name);
        directive_impl.impl_trait("CustomDirective");
        directive_impl.r#macro("#[async_trait::async_trait]");

        let resolve_fn = directive_impl.new_fn("resolve_field");
        resolve_fn.set_async(true);
        resolve_fn.arg_ref_self();
        resolve_fn.arg("ctx", "&Context<'_>");
        resolve_fn.arg("directive_args", "&BTreeMap<String, GqlValue>");
        resolve_fn.arg("resolve_fut", "ResolveFut<'_>");
        resolve_fn.ret("ResolverResult<Option<GqlValue>>");
        resolve_fn.line("todo!()");

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
        self.filename.clone()
    }
}

pub async fn create_directive_files(
    directives: &HashMap<String, DirectiveDefinition>,
    base_path: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut directive_names = Vec::new();
    for (_, directive) in directives.iter() {
        if is_default_directive(&directive.name) {
            continue;
        }
        let filename = &directive.name.to_snake_case();
        let path = path_str(vec![base_path, "directive", filename], true);
        futures.push(create_file(DirectiveFile {
            def: directive,
            path,
            filename: filename.clone(),
        }));
        directive_names.push(directive.name.clone());
    }
    let struct_names = directive_names
        .iter()
        .map(|name| name.to_upper_camel_case())
        .collect::<Vec<_>>();
    ModFile {
        path: &path_str(vec![base_path, "directive"], false),
        struct_names,
    }
    .create_file()
    .await?;

    try_join_all(futures).await
}

fn is_default_directive(name: &str) -> bool {
    vec!["skip", "include", "deprecated"].contains(&name)
}

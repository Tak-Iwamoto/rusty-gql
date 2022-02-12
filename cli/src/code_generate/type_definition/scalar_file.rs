use codegen::Scope;
use rusty_gql::ScalarType;
use tokio::io::AsyncWriteExt;

use crate::code_generate::{use_gql_definitions, CreateFile};

pub struct ScalarFile<'a> {
    pub filename: &'a str,
    pub def: &'a ScalarType,
    pub path: &'a str,
}

#[async_trait::async_trait]
impl<'a> CreateFile for ScalarFile<'a> {
    async fn create_file(&self) -> Result<(), std::io::Error> {
        let path = self.path;
        match tokio::fs::File::open(&path).await {
            Ok(_) => Ok(()),
            Err(_) => {
                let mut file = tokio::fs::File::create(&path).await?;
                file.write(new_file_content(self.def).as_bytes()).await?;
                Ok(())
            }
        }
    }
}

fn new_file_content(scalar_def: &ScalarType) -> String {
    let mut scope = Scope::new();
    let struct_name = &scalar_def.name;
    let scalar_scope = scope.new_struct(struct_name).vis("pub");
    scalar_scope.derive("GqlScalar");

    let scalar_impl = scope.new_impl(struct_name);
    scalar_impl.impl_trait("GqlInputType");
    let from_gql_value_fn = scalar_impl.new_fn("from_gql_value");
    from_gql_value_fn.arg("value", "Option<GqlValue>");
    from_gql_value_fn.ret("Result<Self, String>");
    from_gql_value_fn.line("todo!()");

    let to_gql_value_fn = scalar_impl.new_fn("to_gql_value");
    to_gql_value_fn.arg_ref_self();
    to_gql_value_fn.ret("GqlValue");
    to_gql_value_fn.line("todo!()");

    format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
}

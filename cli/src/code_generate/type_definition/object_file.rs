use codegen::{Scope, Type};
use heck::ToSnakeCase;
use rusty_gql::GqlObject;

use crate::code_generate::{
    type_definition::reserved_scalar_names, use_gql_definitions, util::gql_value_ty_to_rust_ty,
    FileDefinition,
};

pub struct ObjectFile<'a> {
    pub def: &'a GqlObject,
    pub path: &'a str,
    pub interface_names: &'a Vec<String>,
}

impl<'a> FileDefinition for ObjectFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut struct_scope_base = Scope::new();
        let mut impl_scope = Scope::new();
        let struct_name = self.def.name.as_str();
        let struct_scope = struct_scope_base
            .new_struct(&struct_name.to_string())
            .vis("pub");
        let imp = impl_scope.new_impl(&struct_name.to_string());

        for field in &self.def.fields {
            let return_ty = gql_value_ty_to_rust_ty(&field.meta_type);
            let is_gql_primitive_ty = reserved_scalar_names().contains(&field.meta_type.name());
            let field_name = &field.name.to_snake_case();
            if is_gql_primitive_ty {
                struct_scope.field(&field_name, &return_ty);
            }
            let block_str = if is_gql_primitive_ty {
                format!("self.{}.clone()", &field_name)
            } else {
                "todo!()".to_string()
            };

            let f = imp.new_fn(&field_name);
            let mut args_str = String::from("");
            for arg in &field.arguments {
                f.arg(
                    &arg.name.to_snake_case(),
                    gql_value_ty_to_rust_ty(&arg.meta_type),
                );
                args_str += format!("{},", &arg.name.to_snake_case()).as_str();
            }
            // remove last `,`
            args_str.pop();

            f.set_async(true);

            let is_interface_return_ty = self
                .interface_names
                .contains(&field.meta_type.name().to_string());
            if is_interface_return_ty {
                f.generic(&format!("T: {}", &field.meta_type.name()));
                f.ret(Type::new("T"));
            } else {
                f.ret(Type::new(&return_ty));
            }
            f.arg_ref_self();
            f.line(block_str);
        }

        format!(
            "{}\n\n{}\n{}",
            use_gql_definitions(),
            struct_scope_base.to_string(),
            impl_scope.to_string()
        )
    }
}

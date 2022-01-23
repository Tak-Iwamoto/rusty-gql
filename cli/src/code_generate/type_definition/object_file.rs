use codegen::{Scope, Type};
use rusty_gql::{FieldType, ObjectType};

use crate::code_generate::{
    use_gql_definitions,
    util::{gql_value_ty_to_rust_ty, is_gql_primitive_ty},
    FileDefinition,
};

pub struct ObjectFile<'a> {
    pub filename: &'a str,
    pub def: &'a ObjectType,
    pub path: &'a str,
}

impl<'a> FileDefinition for ObjectFile<'a> {
    fn name(&self) -> String {
        self.filename.to_string()
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut struct_scope_base = Scope::new();
        let struct_name = &self.def.name;
        let struct_scope = struct_scope_base
            .new_struct(&struct_name.to_string())
            .vis("pub");
        struct_scope.derive("Clone");

        let mut impl_scope = Scope::new();
        let struct_imp = impl_scope.new_impl(&struct_name.to_string());
        struct_imp.r#macro("#[GqlType]");

        for field in &self.def.fields {
            let field_name = &field.name;
            let return_ty = gql_value_ty_to_rust_ty(&field.meta_type);
            if is_return_primitive_ty(&field) {
                struct_scope.field(format!("pub {}", &field_name).as_str(), &return_ty);
            }

            let fn_scope = struct_imp.new_fn(&field_name);
            for arg in &field.arguments {
                fn_scope.arg(&arg.name, gql_value_ty_to_rust_ty(&arg.meta_type));
            }

            fn_scope.arg_ref_self();
            fn_scope.set_async(true);
            fn_scope.vis("pub");

            fn_scope.ret(Type::new(&return_ty));

            let block_str = build_block_str(&field, &field_name);
            fn_scope.line(block_str);
        }

        format!(
            "{}\n\n{}\n\n{}",
            use_gql_definitions(),
            struct_scope_base.to_string(),
            impl_scope.to_string()
        )
    }
}

fn is_return_primitive_ty(field: &FieldType) -> bool {
    is_gql_primitive_ty(&field.meta_type.name())
}

fn is_copy_gql_ty(field: &FieldType) -> bool {
    vec!["Int", "Float", "Boolean"].contains(&field.meta_type.name())
}

fn build_block_str(field: &FieldType, name: &str) -> String {
    let block_str = if is_return_primitive_ty(&field) {
        if is_copy_gql_ty(&field) {
            format!("self.{}", &name)
        } else {
            format!("self.{}.clone()", &name)
        }
    } else {
        "todo!()".to_string()
    };
    block_str
}

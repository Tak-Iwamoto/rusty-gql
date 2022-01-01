use std::collections::{BTreeMap, HashMap};

use codegen::{Scope, Type};
use rusty_gql::{GqlField, GqlObject};

use crate::code_generate::{
    type_definition::reserved_scalar_names, use_gql_definitions, util::gql_value_ty_to_rust_ty,
    FileDefinition,
};

pub struct ObjectFile<'a> {
    pub def: &'a GqlObject,
    pub path: &'a str,
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
            if reserved_scalar_names().contains(&field.meta_type.name()) {
                struct_scope.field(&field.name, return_ty);
            } else {
                let f = imp.new_fn(&field.name);
                let mut args_str = String::from("");
                for arg in &field.arguments {
                    f.arg(arg.name.as_str(), gql_value_ty_to_rust_ty(&arg.meta_type));
                    args_str += format!("{},", &arg.name).as_str();
                }
                // remove last `,`
                args_str.pop();
                f.set_async(true);
                f.ret(Type::new(&return_ty));
            }
        }

        format!(
            "{}\n\n{}\n\n#[async_trait::async_trait]\n{}",
            use_gql_definitions(),
            struct_scope_base.to_string(),
            impl_scope.to_string()
        )
    }
}

struct StructDefinition<'a> {
    pub fields: &'a Vec<GqlField>,
    pub interface_names: &'a Vec<String>,
}

impl<'a> StructDefinition<'a> {
    // key: generics string, T1, T2.., value: Field
    fn generics_idents(&self) -> HashMap<String, &GqlField> {
        let field_generics_map = self
            .fields
            .iter()
            .filter(|f| {
                self.interface_names
                    .contains(&f.meta_type.name().to_string())
            })
            .enumerate()
            .map(|(i, f)| (format!("T{}", i + 1), f))
            .collect::<Vec<_>>();
        let map: HashMap<String, &GqlField> = field_generics_map.into_iter().collect();
        map
    }

    fn is_return_trait(&self, field: &GqlField) -> bool {
        self.interface_names.contains(&field.name)
    }

    fn build_struct_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        for field in self.fields {
            if self.is_return_trait(field) {
                continue;
            }
            map.insert(
                field.name.to_string(),
                gql_value_ty_to_rust_ty(&field.meta_type),
            );
        }
        for (generics_str, field) in self.generics_idents() {
            map.insert(field.name.to_string(), generics_str);
        }
        map
    }

    fn bound_map(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        for (generics_str, field) in self.generics_idents() {
            map.insert(generics_str, field.meta_type.name().to_string());
        }
        map
    }

    fn generics_str(&self) -> String {
        self.generics_idents()
            .iter()
            .map(|(key, _)| key.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

use std::collections::{BTreeMap, HashMap};

use codegen::Scope;
use rusty_gql::{GqlField, GqlObject};

use crate::code_generate::{use_gql_definitions, FileDefinition};

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
        let mut scope = Scope::new();
        let struct_scope = scope.new_struct(self.def.name.as_str()).vis("pub");

        let st_def = StructDefinition {
            fields: &self.def.fields,
            interface_names: self.interface_names,
        };

        for (field_name, ty_name) in st_def.build_struct_fields() {
            struct_scope.field(&field_name, ty_name);
        }

        if !st_def.generics_idents().is_empty() {
            for (bound_key, bound_ty) in st_def.bound_map() {
                struct_scope.bound(&bound_key, bound_ty);
            }
            struct_scope.generic(&st_def.generics_str());
        }

        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
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
            map.insert(field.name.to_string(), field.meta_type.to_rust_type_str());
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

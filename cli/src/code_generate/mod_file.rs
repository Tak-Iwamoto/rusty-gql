use heck::ToSnakeCase;

use crate::code_generate::FileDefinition;

use super::path_str;

pub struct ModFile<'a> {
    pub struct_names: Vec<String>,
    pub path: &'a str,
}

impl<'a> FileDefinition for ModFile<'a> {
    fn content(&self) -> String {
        let mut mod_str = String::from("");
        let mut pub_use_str = String::from("");
        for name in &self.struct_names {
            mod_str += format!("mod {};\n", &name.to_snake_case()).as_str();
            let file_name = &name.to_snake_case();
            pub_use_str += format!("pub use {}::{};\n", &file_name, &name).as_str();
        }

        format!("{}\n{}", mod_str, pub_use_str)
    }

    fn path(&self) -> String {
        path_str(vec![self.path, "mod"], true)
    }
}

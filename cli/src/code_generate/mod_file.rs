use heck::ToSnakeCase;
use tokio::io::AsyncWriteExt;

use super::{path_str, CreateFile};

pub struct ModFile<'a> {
    pub struct_names: Vec<String>,
    pub path: &'a str,
}

impl<'a> ModFile<'a> {
    fn content(&self) -> String {
        let mut mod_str = String::from("");
        let mut pub_use_str = String::from("");
        for name in &self.struct_names {
            let filename = &name.to_snake_case();
            mod_str += format!("mod {};\n", &filename).as_str();
            pub_use_str += format!("pub use {}::{};\n", &filename, &name).as_str();
        }

        format!("{}\n{}", mod_str, pub_use_str)
    }

    fn path(&self) -> String {
        path_str(vec![self.path, "mod"], true)
    }
}

#[async_trait::async_trait]
impl<'a> CreateFile for ModFile<'a> {
    async fn create_file(&self) -> Result<(), std::io::Error> {
        let path = self.path();
        let mut file = tokio::fs::File::create(&path).await?;
        file.write(self.content().as_bytes()).await?;
        Ok(())
    }
}

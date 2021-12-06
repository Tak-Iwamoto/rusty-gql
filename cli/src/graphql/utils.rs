use std::io::Error;
use tokio::io::AsyncWriteExt;

pub struct PathStr<'a> {
    paths: Vec<&'a str>,
    base_path: Option<&'a str>,
}

impl<'a> PathStr<'a> {
    pub fn new(paths: Vec<&'a str>) -> Self {
        PathStr {
            paths,
            base_path: None,
        }
    }
}

impl<'a> ToString for PathStr<'a> {
    fn to_string(&self) -> String {
        let base_path = match self.base_path {
            Some(path) => path,
            None => "graphql",
        };
        let relative_path = self.paths.join("/");
        format!("{}/{}.rs", base_path, relative_path)
    }
}

pub async fn create_file(path: &str, content: &str) -> Result<(), Error> {
    let mut file = tokio::fs::File::create(&path).await?;
    file.write(content.as_bytes()).await?;
    Ok(())
}

use std::fs;

use anyhow::Result;

fn test() {
    let contents = fs::read_to_string("src/tests/github_query.graphql");
    let v = contents.unwrap();
    build_schema(v.as_str());
}

fn build_schema(schema_doc: &str) -> Result<()> {
    let parsed_schema = graphql_parser::parse_schema::<&str>(schema_doc)?;
    for node in parsed_schema.definitions {
        println!("{:?}", node);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::test;

    #[test]
    fn it_works() {
        test()
    }
}

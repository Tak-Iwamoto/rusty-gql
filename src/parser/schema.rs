use core::num::dec2flt::parse;
use std::fs;

use anyhow::Result;

fn test() {
    let contents = fs::read_to_string("graphql/tests/schema.graphql");
    let v = contents.unwrap();
    parse_schema(v.as_str());
}

fn parse_schema(schema_doc: &str) -> Result<()> {
    let parsed_schema = graphql_parser::parse_schema::<&str>(schema_doc)?;
    println!("{:?}", parsed_schema.definitions);
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

use core::panic;
use std::collections::HashMap;

use graphql_parser::query::{Document, FragmentDefinition};

use crate::{build_schema, operation::build_operation, types::schema::ArcSchema};

use super::visitor::{visit, ValidationContext, ValidationError, Visitor};

pub(crate) fn validate<'a, V, F>(
    doc: &'a Document<'a, String>,
    schema: &'a ArcSchema,
    fragments: &'a HashMap<String, FragmentDefinition<'a, String>>,
    factory: F,
) -> Result<(), Vec<ValidationError>>
where
    V: Visitor<'a> + 'a,
    F: Fn() -> V,
{
    let mut ctx = ValidationContext::new(&schema, &doc, None, &fragments);
    let mut visitor = factory();
    visit(&mut visitor, &mut ctx, &doc, None);

    if ctx.errors.is_empty() {
        Ok(())
    } else {
        Err(ctx.errors)
    }
}

pub(crate) fn assert_passes_rule<'a, V, F>(
    doc: &'a Document<'a, String>,
    schema: &'a ArcSchema,
    fragments: &'a HashMap<String, FragmentDefinition<'a, String>>,
    factory: F,
) where
    V: Visitor<'a> + 'a,
    F: Fn() -> V,
{
    if let Err(errors) = validate(doc, schema, fragments, factory) {
        for err in errors {
            if let Some(pos) = err.locations.first() {
                println!("[{}:{}]", pos.line, pos.column);
            }
            println!("{}", err.message);
        }
        panic!("The rule passes, but errors found");
    }
}

pub(crate) fn assert_fails_rule<'a, V, F>(
    doc: &'a Document<'a, String>,
    schema: &'a ArcSchema,
    fragments: &'a HashMap<String, FragmentDefinition<'a, String>>,
    factory: F,
) where
    V: Visitor<'a> + 'a,
    F: Fn() -> V,
{
    if validate(doc, schema, fragments, factory).is_ok() {
        panic!("should fail, but the rule passes");
    }
}

pub(crate) fn test_schema() -> ArcSchema {
    let contents = std::fs::read_to_string("tests/schemas/starwars.graphql").unwrap();
    let schema = build_schema(&vec![contents.as_str()]).unwrap();
    ArcSchema::new(schema)
}

pub(crate) fn parse_test_query<'a>(query_doc: &'a str) -> Document<'a, String> {
    graphql_parser::parse_query::<String>(query_doc).unwrap()
}

pub(crate) fn get_query_fragment_definitions<'a>(
    doc: &'a Document<'a, String>,
    schema: &'a ArcSchema,
) -> HashMap<String, FragmentDefinition<'a, String>> {
    build_operation(&doc, None, Default::default(), schema)
        .unwrap()
        .fragment_definitions
}

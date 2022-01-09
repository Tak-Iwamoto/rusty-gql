use core::panic;

use graphql_parser::query::Document;

use crate::{
    build_schema,
    operation::{build_operation, Operation},
    types::schema::ArcSchema,
};

use super::visitor::{visit, ValidationContext, ValidationError, Visitor};

pub(crate) fn validate<'a, V, F>(
    doc: &'a Document<'a, String>,
    schema: &'a ArcSchema,
    operation: &'a Operation<'a>,
    factory: F,
) -> Result<(), Vec<ValidationError>>
where
    V: Visitor<'a> + 'a,
    F: Fn() -> V,
{
    let mut ctx = ValidationContext::new(&schema, None, &operation);
    let mut visitor = factory();
    visit(&mut visitor, &mut ctx, &doc, None);

    if ctx.errors.is_empty() {
        Ok(())
    } else {
        Err(ctx.errors)
    }
}

#[macro_export]
macro_rules! check_passes_rule {
    ($query_doc: expr, $factory: expr $(,)?) => {
        let schema = &crate::validation::test_utils::test_schema();
        let doc = &crate::validation::test_utils::parse_test_query($query_doc);
        let operation = crate::validation::test_utils::build_test_operation(doc);
        crate::validation::test_utils::assert_passes_rule(doc, schema, &operation, $factory);
    };
}
#[macro_export]
macro_rules! check_fails_rule {
    ($query_doc: expr, $factory: expr $(,)?) => {
        let schema = &crate::validation::test_utils::test_schema();
        let doc = &crate::validation::test_utils::parse_test_query($query_doc);
        let operation = crate::validation::test_utils::build_test_operation(doc);
        crate::validation::test_utils::assert_fails_rule(doc, schema, &operation, $factory);
    };
}

pub(crate) fn assert_passes_rule<'a, V, F>(
    doc: &'a Document<'a, String>,
    schema: &'a ArcSchema,
    operation: &'a Operation<'a>,
    factory: F,
) where
    V: Visitor<'a> + 'a,
    F: Fn() -> V,
{
    if let Err(errors) = validate(doc, schema, operation, factory) {
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
    operation: &'a Operation<'a>,
    factory: F,
) where
    V: Visitor<'a> + 'a,
    F: Fn() -> V,
{
    if validate(doc, schema, operation, factory).is_ok() {
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

pub(crate) fn build_test_operation<'a>(doc: &'a Document<'a, String>) -> Operation<'a> {
    build_operation(&doc, None, Default::default()).unwrap()
}

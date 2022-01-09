use core::panic;
use std::collections::HashMap;

use graphql_parser::query::{Document, Field, FragmentDefinition};

use crate::{
    build_schema,
    operation::{build_operation, Operation},
    types::schema::ArcSchema,
};

use super::visitor::{visit, ValidationContext, ValidationError, Visitor};

pub(crate) fn validate<'a, V, F>(
    doc: &'a Document<'a, String>,
    schema: &'a ArcSchema,
    fragments: &'a HashMap<String, FragmentDefinition<'a, String>>,
    root_field: &'a Field<'a, String>,
    factory: F,
) -> Result<(), Vec<ValidationError>>
where
    V: Visitor<'a> + 'a,
    F: Fn() -> V,
{
    let mut ctx = ValidationContext::new(&schema, &doc, None, &fragments, root_field);
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
        let operation = crate::validation::test_utils::build_test_operation(doc, schema);
        crate::validation::test_utils::assert_passes_rule(
            doc,
            schema,
            &operation.fragment_definitions,
            &operation.root_field,
            $factory,
        );
    };
}

#[macro_export]
macro_rules! check_fails_rule {
    ($query_doc: expr, $factory: expr $(,)?) => {
        let schema = &crate::validation::test_utils::test_schema();
        let doc = &crate::validation::test_utils::parse_test_query($query_doc);
        let operation = crate::validation::test_utils::build_test_operation(doc, schema);
        crate::validation::test_utils::assert_fails_rule(
            doc,
            schema,
            &operation.fragment_definitions,
            &operation.root_field,
            $factory,
        );
    };
}

pub(crate) fn assert_passes_rule<'a, V, F>(
    doc: &'a Document<'a, String>,
    schema: &'a ArcSchema,
    fragments: &'a HashMap<String, FragmentDefinition<'a, String>>,
    root_field: &'a Field<'a, String>,
    factory: F,
) where
    V: Visitor<'a> + 'a,
    F: Fn() -> V,
{
    if let Err(errors) = validate(doc, schema, fragments, root_field, factory) {
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
    root_field: &'a Field<'a, String>,
    factory: F,
) where
    V: Visitor<'a> + 'a,
    F: Fn() -> V,
{
    if validate(doc, schema, fragments, root_field, factory).is_ok() {
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

pub(crate) fn build_test_operation<'a>(
    doc: &'a Document<'a, String>,
    schema: &'a ArcSchema,
) -> Operation<'a> {
    build_operation(&doc, None, Default::default(), schema).unwrap()
}

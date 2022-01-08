use std::collections::HashMap;

use graphql_parser::query::{Document, FragmentDefinition};

use crate::types::schema::ArcSchema;

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

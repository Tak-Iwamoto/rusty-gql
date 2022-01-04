use graphql_parser::query::Document;

use crate::{GqlError, Schema, Variables};

use self::visitor::ValidationContext;

mod rules;
mod utils;
mod visitor;

pub fn apply_validation<'a>(
    schema: &'a Schema,
    query_doc: &'a Document<'a, String>,
) -> Result<(), Vec<GqlError>> {
    // let mut ctx = ValidationContext::new(schema, query_doc);

    // if !ctx.errors.is_empty() {
    //     return Err(ctx.errors.into_iter().map(|v| v.into()).collect());
    // }
    Ok(())
}

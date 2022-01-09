use std::collections::HashMap;

use graphql_parser::query::{Document, Field, FragmentDefinition};

use crate::{operation::Operation, types::schema::ArcSchema, GqlError, Schema, Variables};

use self::visitor::{visit, NewVisitor, ValidationContext};

mod rules;
mod test_utils;
mod utils;
mod visitor;

pub fn apply_validation<'a>(
    schema: &'a ArcSchema,
    query_doc: &'a Document<'a, String>,
    variables: Option<&'a Variables>,
    operation: &'a Operation<'a>,
    operation_name: Option<&'a str>,
) -> Result<(), Vec<GqlError>> {
    let mut ctx = ValidationContext::new(schema, query_doc, variables, operation);
    let mut visitor = NewVisitor
        .with(rules::DefaultValueOfCorrectType::default())
        .with(rules::FieldsOnCorrectType::default())
        .with(rules::FragmentsOnCompositeTypes::default())
        .with(rules::KnownArgumentNames::default())
        .with(rules::KnownDirectives::default())
        .with(rules::KnownFragmentName::default())
        .with(rules::KnownTypeNames::default())
        .with(rules::NoFragmentCycles::default())
        .with(rules::NoUndefinedVariables::default())
        .with(rules::NoUnusedFragment::default())
        .with(rules::NoUnusedVariables::default())
        .with(rules::OverlappingFieldsCanBeMerged::default())
        .with(rules::PossibleFragmentSpreads::default())
        .with(rules::ProvidedNonNullArguments::default())
        .with(rules::ScalarLeafs::default())
        .with(rules::UniqueArgumentNames::default())
        .with(rules::UniqueVariableNames::default())
        .with(rules::VariablesAreInputTypes::default())
        .with(rules::VariablesInAllowedPosition::default());
    // .with(rules::ArgumentsOfCorrectType::default())

    visit(&mut visitor, &mut ctx, query_doc, operation_name);

    if !ctx.errors.is_empty() {
        return Err(ctx.errors.into_iter().map(|v| v.into()).collect());
    }

    Ok(())
}

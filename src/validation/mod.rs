use graphql_parser::query::Document;

use crate::{GqlError, Schema, Variables};

use self::visitor::{NewVisitor, ValidationContext};

mod rules;
mod utils;
mod visitor;

pub fn apply_validation<'a>(
    schema: &'a Schema,
    query_doc: &'a Document<'a, String>,
    variables: Option<&Variables>
) -> Result<(), Vec<GqlError>> {
    let mut visitor = NewVisitor
        .with(rules::ArgumentsOfCorrectType::default())
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
    // let mut ctx = ValidationContext::new(schema, query_doc);

    // if !ctx.errors.is_empty() {
    //     return Err(ctx.errors.into_iter().map(|v| v.into()).collect());
    // }
    Ok(())
}

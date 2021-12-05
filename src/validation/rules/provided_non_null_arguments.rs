use graphql_parser::{query::Field, schema::Directive};

use crate::{
    validation::{
        utils::get_type_name,
        visitor::{ValidationContext, Visitor},
    },
    GqlTypeDefinition,
};

pub struct ProvidedNonNullArguments;

impl<'a> Visitor<'a> for ProvidedNonNullArguments {
    fn enter_directive(
        &mut self,
        ctx: &mut ValidationContext,
        directive: &'a Directive<'a, String>,
    ) {
        if let Some(schema_directive) = ctx.schema.directives.get(&directive.name) {
            for arg in &schema_directive.arguments {
                if arg.meta_type.is_non_null()
                    && arg.default_value.is_none()
                    && !directive
                        .arguments
                        .iter()
                        .any(|(name, _)| name.eq(&arg.name))
                {
                    ctx.add_error(
                        format!(
                            "Directive @{} argument {} of type {} is required but not provided",
                            directive.name,
                            arg.name,
                            get_type_name(&arg.meta_type.to_parser_type())
                        ),
                        vec![directive.position],
                    )
                }
            }
        }
    }

    fn enter_field(&mut self, ctx: &mut ValidationContext, field: &'a Field<'a, String>) {
        if let Some(parent_type) = ctx.parent_type() {
            if let Some(parent_gql_type) = ctx
                .schema
                .type_definitions
                .get(&GqlTypeDefinition::type_name_from_def(parent_type))
            {
                if let Some(target_field) = parent_gql_type.get_field_by_name(&field.name) {
                    for arg in &target_field.arguments {
                        if arg.meta_type.is_non_null()
                            && !field.arguments.iter().any(|(name, _)| name.eq(&arg.name))
                        {
                            ctx.add_error(
                                format!(
                                    "Field {} argument {} of type {} is required but not provided",
                                    field.name,
                                    arg.name,
                                    GqlTypeDefinition::type_name_from_def(parent_type),
                                ),
                                vec![field.position],
                            )
                        }
                    }
                }
            }
        }
    }
}

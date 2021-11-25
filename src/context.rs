use crate::{
    error::GqlError, operation::ArcOperation, path::GraphQLPath, types::schema::ArcSchema,
};
use graphql_parser::{
    query::{Field, SelectionSet},
    schema::Directive,
};

#[derive(Debug, Clone)]
pub struct ExecutionContext<'a, T> {
    pub schema: &'a ArcSchema,
    pub operation: &'a ArcOperation<'a>,
    pub item: T,
    pub current_path: GraphQLPath,
    pub errors: Vec<GqlError>,
}

pub type FieldContext<'a> = ExecutionContext<'a, &'a Field<'a, String>>;

pub type SelectionSetContext<'a> = ExecutionContext<'a, &'a SelectionSet<'a, String>>;

impl<'a, T> ExecutionContext<'a, T> {
    pub fn with_field(
        &self,
        field: &'a Field<'a, String>,
    ) -> ExecutionContext<'a, &'a Field<'a, String>> {
        ExecutionContext {
            schema: self.schema,
            operation: self.operation,
            item: field,
            current_path: self.current_path.clone(),
            errors: self.errors.clone(),
        }
    }

    pub fn with_selection_set(
        &self,
        selection_set: &'a SelectionSet<'a, String>,
    ) -> ExecutionContext<'a, &'a SelectionSet<'a, String>> {
        ExecutionContext {
            schema: self.schema,
            operation: self.operation,
            item: selection_set,
            current_path: self.current_path.clone(),
            errors: self.errors.clone(),
        }
    }

    pub fn is_skip(&self, directives: &'a [Directive<'a, String>]) -> bool {
        for dir in directives {
            let skip = match dir.name.as_str() {
                "skip" => true,
                "include" => false,
                _ => continue,
            };
            return skip;
        }
        false
    }
}

pub(crate) fn build_context<'a>(
    schema: &'a ArcSchema,
    operation: &'a ArcOperation<'a>,
) -> ExecutionContext<'a, &'a Field<'a, String>> {
    let operation_type = operation.operation_type.to_string();
    let root_fieldname = operation.root_field.name.to_string();
    let current_field = &operation.root_field;

    let current_path = GraphQLPath::default()
        .prev(None)
        .current_key(root_fieldname)
        .parent_name(operation_type);

    ExecutionContext {
        schema,
        operation,
        item: current_field,
        current_path,
        errors: vec![],
    }
}

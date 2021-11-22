use crate::{
    error::GqlError, operation::ArcOperation, path::GraphQLPath, types::schema::ArcSchema,
};
use graphql_parser::{
    query::{Field, SelectionSet},
    schema::Directive,
};

#[derive(Debug, Clone)]
pub struct ExecutionContext<'a> {
    pub schema: &'a ArcSchema,
    pub operation: &'a ArcOperation<'a>,
    pub current_field: Field<'a, String>,
    pub selection_set: SelectionSet<'a, String>,
    pub current_path: GraphQLPath,
    pub errors: Vec<GqlError>,
}

impl<'a> ExecutionContext<'a> {
    pub fn current_field(&self, field: Field<'a, String>) -> Self {
        ExecutionContext {
            schema: self.schema,
            operation: self.operation,
            current_field: field,
            selection_set: self.selection_set.clone(),
            current_path: self.current_path.clone(),
            errors: self.errors.clone(),
        }
    }

    pub fn current_selection_set(&self, selection_set: &SelectionSet<'a, String>) -> Self {
        ExecutionContext {
            schema: self.schema,
            operation: self.operation,
            current_field: self.current_field.clone(),
            selection_set: selection_set.clone(),
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
) -> ExecutionContext<'a> {
    let operation_type = operation.operation_type.to_string();
    let root_fieldname = operation.root_field.name.to_string();
    let current_field = operation.root_field.clone();
    let selection_set = current_field.selection_set.clone();

    let current_path = GraphQLPath::default()
        .prev(None)
        .current_key(root_fieldname)
        .parent_name(operation_type);

    ExecutionContext {
        schema,
        operation,
        current_field,
        selection_set,
        current_path,
        errors: vec![],
    }
}

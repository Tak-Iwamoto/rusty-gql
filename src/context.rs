use crate::{
    error::GqlError, input::VariableType, operation::Operation, path::GqlPath,
    types::schema::Schema, GqlValue, ResolverResult,
};
use graphql_parser::{
    query::{Field, SelectionSet},
    schema::{Directive, Value},
};

#[derive(Clone)]
pub struct ExecutionContext<'a, T> {
    pub schema: &'a Schema,
    pub operation: &'a Operation<'a>,
    pub item: T,
    pub current_path: GqlPath,
}

pub type FieldContext<'a> = ExecutionContext<'a, &'a Field<'a, String>>;

impl<'a> FieldContext<'a> {
    pub fn get_arg_value<T: VariableType>(&self, arg_name: &str) -> ResolverResult<T> {
        let value = self
            .item
            .arguments
            .iter()
            .find(|(name, _)| name == arg_name)
            .map(|(_, v)| v);
        let gql_value = match value {
            Some(v) => {
                if let Value::Variable(var_name) = v {
                    self.resolve_variable_value(var_name)?
                } else {
                    GqlValue::from(v.clone())
                }
            }
            None => GqlValue::Null,
        };
        match T::from_gql_value(Some(gql_value)) {
            Ok(v) => Ok(v),
            Err(err) => Err(GqlError::new(err, None)),
        }
    }
}

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
        }
    }

    pub fn is_skip(&self, directives: &'a [Directive<'a, String>]) -> bool {
        for dir in directives {
            let skip = match dir.name.as_str() {
                "skip" => true,
                "include" => false,
                _ => continue,
            };

            for (key, value) in &dir.arguments {
                if key != "if" {
                    continue;
                } else {
                    if let Value::Boolean(cond) = value {
                        if skip && *cond {
                            return true;
                        }
                    }
                }
            }

            return skip;
        }

        false
    }
    pub fn add_error(&self, error: &GqlError) {
        self.operation.errors.lock().unwrap().push(error.clone());
    }

    pub fn resolve_variable_value(&self, name: &str) -> ResolverResult<GqlValue> {
        let v = self
            .operation
            .variable_definitions
            .iter()
            .find(|var_def| var_def.name == name)
            .and_then(|var_def| self.operation.variables.0.get(&var_def.name));
        match v {
            Some(value) => Ok(value.clone()),
            None => Err(GqlError::new(
                format!("Variable {} is not defined", name),
                None,
            )),
        }
    }
}

pub(crate) fn build_context<'a>(
    schema: &'a Schema,
    operation: &'a Operation<'a>,
) -> ExecutionContext<'a, &'a SelectionSet<'a, String>> {
    let operation_type = operation.operation_type.to_string();

    let current_path = GqlPath::default().prev(None).parent_name(operation_type);

    ExecutionContext {
        schema,
        operation,
        item: &operation.selection_set,
        current_path,
    }
}

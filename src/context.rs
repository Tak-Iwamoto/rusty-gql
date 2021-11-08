use crate::{
    graphql_object::GraphQLObject,
    graphql_value::{value_from_ast, GraphQLValue},
    operation::Operation,
    path::GraphQLPath,
    types::GraphQLType,
    Schema,
};
use graphql_parser::{
    query::{Field, Selection, SelectionSet},
    schema::Type,
};
use std::collections::{BTreeMap, HashMap, HashSet};

pub struct ExecutionContext<'a> {
    pub schema: &'a Schema<'a>,
    pub operation: &'a Operation<'a>,
    pub fields: BTreeMap<String, Vec<Field<'a, &'a str>>>,
    pub current_field: Field<'a, &'a str>,
    pub current_path: GraphQLPath,
}

pub fn build_context<'a>(
    schema: &'a Schema<'a>,
    operation: &'a Operation<'a>,
) -> ExecutionContext<'a> {
    let fields = collect_all_fields(schema, operation, &operation.definition.selection_set);
    let current_field = operation.definition.root_field.clone();
    let current_path = GraphQLPath::default()
        .prev(None)
        .key(operation.definition.root_field.name.to_string())
        .parent_name(operation.definition.operation_type.to_string());

    ExecutionContext {
        schema,
        operation,
        fields,
        current_field,
        current_path,
    }
}

pub fn get_variables<'a>(
    schema: &'a Schema<'a>,
    operation: &'a Operation<'a>,
    input_values: &BTreeMap<String, GraphQLValue>,
) -> Result<HashMap<String, GraphQLValue>, String> {
    let variable_definitions = &operation.definition.variable_definitions;
    let mut variables = HashMap::new();
    for var in variable_definitions {
        let var_type = get_type_from_schema(schema, &var.var_type);
        if var_type.is_none() {
            continue;
        }
        let var_type = var_type.unwrap();

        let var_name = &var.name.to_string();
        if !input_values.contains_key(var_name) {
            if let Some(value) = &var.default_value {
                variables.insert(
                    var.name.to_string(),
                    value_from_ast(value, &var_type, &None),
                );
            }
        }

        let value = input_values.get(var_name);

        if let GraphQLType::NonNull(_) = var_type {
            if value.is_none() {
                return Err(format!("{} must not be null", var_name));
            }
        }

        if let Some(var_value) = value {
            variables.insert(var_name.to_string(), var_value.clone());
        }
    }
    Ok(variables)
}

pub fn get_arguments<'a>(
    field: Field<'a, &'a str>,
    variable_values: HashMap<String, GraphQLValue>,
) {
    let arguments = field.arguments;
}

pub fn get_type_from_schema<'a>(
    schema: &'a Schema<'a>,
    var_type: &'a Type<'a, &'a str>,
) -> Option<GraphQLType<'a>> {
    match var_type {
        graphql_parser::schema::Type::NamedType(named_type) => {
            return schema
                .type_map
                .get(&named_type.to_string())
                .map(|var_ty| var_ty.clone())
        }
        graphql_parser::schema::Type::ListType(list) => {
            let inner_type = get_type_from_schema(schema, &list).unwrap();
            let value = GraphQLType::List(Box::new(inner_type.clone()));
            return Some(value);
        }
        graphql_parser::schema::Type::NonNullType(non_null) => {
            let inner_type = get_type_from_schema(schema, &non_null).unwrap();
            let value = GraphQLType::NonNull(Box::new(inner_type.clone()));
            return Some(value);
        }
    }
}

fn execute_fields<'a>(
    ctx: &ExecutionContext,
    parent_type: &GraphQLObject,
    fields: Vec<Field<'a, &'a str>>,
) {
    // let field_def = get_field_def()
}

fn get_field_def<'a>(parent_type: &GraphQLObject, field: Field<'a, &'a str>) {}

// TODO: schemaはfragmentの条件やskip directiveの処理で使用する
pub fn collect_all_fields<'a>(
    schema: &'a Schema,
    operation: &'a Operation<'a>,
    selection_set: &SelectionSet<'a, &'a str>,
) -> BTreeMap<String, Vec<Field<'a, &'a str>>> {
    let mut fields: BTreeMap<String, Vec<Field<&str>>> = BTreeMap::new();
    let mut visited_fragments = HashSet::new();

    collect_fields(
        operation,
        &selection_set,
        &mut fields,
        &mut visited_fragments,
    );
    fields
}

fn collect_fields<'a>(
    operation: &'a Operation<'a>,
    selection_set: &SelectionSet<'a, &'a str>,
    fields: &mut BTreeMap<String, Vec<Field<'a, &'a str>>>,
    visited_fragments: &mut HashSet<&'a str>,
) {
    for item in &selection_set.items {
        match item {
            Selection::Field(field) => {
                if fields.contains_key(&field.name.to_string()) {
                    fields
                        .get_mut(&field.name.to_string())
                        .unwrap()
                        .push(field.clone());
                } else {
                    fields.insert(field.name.to_string(), vec![field.clone()]);
                }
            }
            Selection::FragmentSpread(spread_frg) => {
                let fragment_name = spread_frg.fragment_name;
                if visited_fragments.contains(fragment_name) {
                    continue;
                }
                visited_fragments.insert(fragment_name);
                let fragment = operation.fragments.get(fragment_name);
                match fragment {
                    Some(frg) => {
                        return collect_fields(
                            operation,
                            &frg.selection_set,
                            fields,
                            visited_fragments,
                        );
                    }
                    None => continue,
                }
            }
            Selection::InlineFragment(inline_frg) => {
                collect_fields(
                    operation,
                    &inline_frg.selection_set,
                    fields,
                    visited_fragments,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        context::collect_all_fields, operation::build_operation, types::schema::build_schema,
    };
    use std::fs;

    #[test]
    fn it_works() {
        let schema_doc = fs::read_to_string("src/tests/github.graphql").unwrap();
        let query_doc = fs::read_to_string("src/tests/github_query.graphql").unwrap();

        let schema = build_schema(schema_doc.as_str()).unwrap();
        let query = build_operation(query_doc.as_str(), &schema, None).unwrap();

        let fields = collect_all_fields(&schema, &query, &query.definition.selection_set);

        for f in &fields["repository"] {
            for item in &f.selection_set.items {
                println!("{:?}", item);
            }
        }
    }
}

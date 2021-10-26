use std::collections::{BTreeMap, HashSet};

use graphql_parser::query::{Field, Selection, SelectionSet};

use crate::{operation::GraphQLOperation, GraphQLSchema};

pub struct ExecutorContext<'a> {
    pub schema: &'a GraphQLSchema,
    pub operation: &'a GraphQLOperation<'a>,
    pub fields: BTreeMap<String, Vec<Field<'a, &'a str>>>,
}

pub fn build_context<'a>(
    schema: &'a GraphQLSchema,
    operation: &'a GraphQLOperation<'a>,
) -> ExecutorContext<'a> {
    let fields = collect_all_fields(schema, operation);
    ExecutorContext {
        schema,
        operation,
        fields,
    }
}

fn collect_all_fields<'a>(
    schema: &'a GraphQLSchema,
    operation: &'a GraphQLOperation<'a>,
) -> BTreeMap<String, Vec<Field<'a, &'a str>>> {
    let mut fields: BTreeMap<String, Vec<Field<&str>>> = BTreeMap::new();
    let mut visited_fragments = HashSet::new();

    collect_fields(
        operation,
        &operation.definition.selection_set,
        &mut fields,
        &mut visited_fragments,
    );
    fields
}

fn collect_fields<'a>(
    operation: &'a GraphQLOperation<'a>,
    selection_set: &SelectionSet<'a, &'a str>,
    fields: &mut BTreeMap<String, Vec<Field<'a, &'a str>>>,
    visited_fragments: &mut HashSet<&'a str>,
) {
    for item in &selection_set.items {
        match item {
            Selection::Field(field) => match fields.get(&field.name.to_string()) {
                Some(_) => {
                    fields
                        .get_mut(&field.name.to_string())
                        .unwrap()
                        .push(field.clone());
                }
                None => {
                    fields.insert(field.name.to_string(), vec![field.clone()]);
                }
            },
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
        executor::collect_all_fields, operation::build_operation, types::schema::build_schema,
    };
    use std::fs;

    #[test]
    fn it_works() {
        let schema_doc = fs::read_to_string("src/tests/github.graphql").unwrap();
        let query_doc = fs::read_to_string("src/tests/github_query.graphql").unwrap();

        let schema = build_schema(schema_doc.as_str()).unwrap();
        let query = build_operation(query_doc.as_str(), None).unwrap();

        let fields = collect_all_fields(&schema, &query);

        for f in &fields["repositories"] {
            for item in &f.selection_set.items {
                println!("{:?}", item);
            }
        }
    }
}

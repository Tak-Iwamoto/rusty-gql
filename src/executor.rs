use std::collections::{BTreeMap, HashSet};

use graphql_parser::query::{Field, Selection, SelectionSet};

use crate::{
    operation::{GraphQLOperation, OperationRequest},
    resolver::Resolver,
    GraphQLError, GraphQLSchema,
};

pub struct ExecutorContext<'a> {
    schema: GraphQLSchema,
    operation: GraphQLOperation<'a>,
}

impl<'a> ExecutorContext<'a> {
    pub fn collect_all_fields(&'a self) -> BTreeMap<String, Vec<Field<&str>>> {
        let mut fields: BTreeMap<String, Vec<Field<&str>>> = BTreeMap::new();
        let mut visited_fragments = HashSet::new();

        match &self.operation.operation_request {
            OperationRequest::Single(single_request) => {
                self.collect_fields(
                    &single_request.selection_set,
                    &mut fields,
                    &mut visited_fragments,
                );
            }

            OperationRequest::Multi(requests) => {
                for operation_def in requests {
                    self.collect_fields(
                        &operation_def.selection_set,
                        &mut fields,
                        &mut visited_fragments,
                    );
                }
            }
        }
        fields
    }

    fn collect_fields(
        &'a self,
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
                    let fragment = self.operation.fragments.get(fragment_name);
                    match fragment {
                        Some(frg) => {
                            return self.collect_fields(
                                &frg.selection_set,
                                fields,
                                visited_fragments,
                            );
                        }
                        None => continue,
                    }
                }
                Selection::InlineFragment(inline_frg) => {
                    self.collect_fields(&inline_frg.selection_set, fields, visited_fragments);
                }
            }
        }
    }
}

pub struct Executor<'a> {
    context: ExecutorContext<'a>,
    field_resolver: Box<dyn Resolver>,
    type_resolver: Box<dyn Resolver>,
    errors: Vec<GraphQLError>,
}

impl<'a> Executor<'a> {
    pub fn execute_fields() {}
}

#[cfg(test)]
mod tests {
    use crate::{operation::build_operation, types::schema::build_schema};
    use std::fs;

    use super::ExecutorContext;

    #[test]
    fn it_works() {
        let schema_doc = fs::read_to_string("src/tests/github.graphql").unwrap();
        let query_doc = fs::read_to_string("src/tests/github_query.graphql").unwrap();

        let schema = build_schema(schema_doc.as_str());
        let query = build_operation(query_doc.as_str(), None);

        let context = ExecutorContext {
            schema: schema.unwrap(),
            operation: query.unwrap(),
        };

        let fields = context.collect_all_fields();

        println!("{}", &fields.len());

        for f in &fields["articles"] {
            for item in &f.selection_set.items {
                println!("{:?}", item);
            }
        }
    }
}

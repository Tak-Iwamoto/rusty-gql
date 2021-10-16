use std::collections::{BTreeMap, HashSet};

use graphql_parser::query::{Field, Selection, SelectionSet};

use crate::{
    operation::{GQLOperationDefinition, GraphQLOperation, OperationRequest},
    resolver::Resolver,
    types::GraphQLFragmentDefinition,
    GraphQLError, GraphQLSchema,
};

pub struct ExecutorContext<'a> {
    schema: GraphQLSchema,
    operation: GraphQLOperation<'a>,
}

impl<'a> ExecutorContext<'a> {
    pub fn collect_fields(
        &'a self,
        selection_set: Option<&SelectionSet<'a, &'a str>>,
    ) -> BTreeMap<String, Vec<Field<&str>>> {
        let mut fields: BTreeMap<String, Vec<Field<&str>>> = BTreeMap::new();
        let mut visited_fragments = HashSet::new();

        // TODO: refactoring
        match &self.operation.operation_request {
            OperationRequest::Single(single_request) => {
                for item in &single_request.selection_set.items {
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
                                Some(frg) => return self.collect_fields(Some(&frg.selection_set)),
                                None => continue,
                            }
                        }
                        Selection::InlineFragment(inline_frg) => {
                            self.collect_fields(Some(&inline_frg.selection_set));
                        }
                    }
                }
            }
            OperationRequest::Multi(requests) => {
                for operation_def in requests {
                    for item in &operation_def.selection_set.items {
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
                                        return self.collect_fields(Some(&frg.selection_set))
                                    }
                                    None => continue,
                                }
                            }
                            Selection::InlineFragment(inline_frg) => {
                                self.collect_fields(Some(&inline_frg.selection_set));
                            }
                        }
                    }
                }
            }
        }
        fields
    }
}

pub struct Executor {
    schema: GraphQLSchema,
    fragments: HashSet<String, GraphQLFragmentDefinition>,
    // 一旦valueをstringにする
    variables: HashSet<String, String>,
    field_resolver: Box<dyn Resolver>,
    type_resolver: Box<dyn Resolver>,
    errors: Vec<GraphQLError>,
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

        let fields = context.collect_fields(None);

        for field in fields {
            println!("{:?}", field);
        }
    }
}

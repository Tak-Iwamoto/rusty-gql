use std::collections::{HashMap, HashSet};

use anyhow::Result;
use graphql_parser::query::{Field, FragmentDefinition, Selection, SelectionSet};

use crate::GraphQLSchema;

enum GraphQLOperationType {
    Query,
    Mutation,
    Subscription,
}

//graphql-jsのbuildExecutionContextが参考になりそう
fn build_operation(doc: &str) -> Result<()> {
    let mut fragment_map: HashMap<String, FragmentDefinition<&str>> = HashMap::new();
    let parsed_query = graphql_parser::parse_query::<&str>(doc)?;
    let first_def = &parsed_query.definitions[0];
    let operation_type = match first_def {
        graphql_parser::query::Definition::Operation(operation) => match operation {
            graphql_parser::query::OperationDefinition::SelectionSet(_) => {
                GraphQLOperationType::Query
            }
            graphql_parser::query::OperationDefinition::Query(_) => GraphQLOperationType::Query,
            graphql_parser::query::OperationDefinition::Mutation(_) => {
                GraphQLOperationType::Mutation
            }
            graphql_parser::query::OperationDefinition::Subscription(_) => {
                GraphQLOperationType::Subscription
            }
        },
        graphql_parser::query::Definition::Fragment(_) => unreachable!(),
    };

    for node in parsed_query.definitions {
        match node {
            graphql_parser::query::Definition::Operation(operation) => match operation {
                graphql_parser::query::OperationDefinition::SelectionSet(selection) => {
                    let fields_map = collect_fields(selection);
                    let fields = fields_map.get("viewer").unwrap();

                    for f in fields {
                        println!("{:?}", "field");
                    }
                }
                graphql_parser::query::OperationDefinition::Query(query) => {
                    let fields = collect_fields(query.selection_set);
                }
                graphql_parser::query::OperationDefinition::Mutation(mutation) => {
                    let fields = collect_fields(mutation.selection_set);
                }
                graphql_parser::query::OperationDefinition::Subscription(sub) => {
                    let fields = collect_fields(sub.selection_set);
                }
            },
            graphql_parser::query::Definition::Fragment(fragment) => {
                let name = fragment.name.to_string();
                fragment_map.insert(name, fragment);
            }
        }
    }
    Ok(())
}

fn collect_fields<'a>(
    fragments: HashMap<String, FragmentDefinition<'a, &'a str>>,
    selection_set: SelectionSet<'a, &'a str>,
) -> HashMap<String, Vec<Field<'a, &'a str>>> {
    let mut fields_map: HashMap<String, Vec<Field<&str>>> = HashMap::new();
    let mut visited_fragments = HashSet::new();
    for item in selection_set.items {
        match item {
            Selection::Field(field) => match fields_map.get(&field.name.to_string()) {
                Some(_) => {
                    fields_map
                        .get_mut(&field.name.to_string())
                        .unwrap()
                        .push(field);
                }
                None => {
                    fields_map.insert(field.name.to_string(), vec![field]);
                }
            },
            Selection::FragmentSpread(spread_frg) => {
                let fragment_name = spread_frg.fragment_name;
                if visited_fragments.contains(fragment_name.to_string()) {
                    continue;
                }
                visited_fragments.insert(fragment_name);
                let fragment = fragments.get(fragment_name);
                match fragment {
                    Some(frg) => collect_fields(fragments, frg.selection_set),
                    None => continue,
                }
            }
            Selection::InlineFragment(inline_frg) => {
                collect_fields(inline_frg.selection_set);
            }
        }
    }
    fields_map
}

fn should_include_node() {}
// function shouldIncludeNode(
//     variableValues: { [variable: string]: unknown },
//     node: FragmentSpreadNode | FieldNode | InlineFragmentNode,
//   ): boolean {
//     const skip = getDirectiveValues(GraphQLSkipDirective, node, variableValues);
//     if (skip?.if === true) {
//       return false;
//     }

//     const include = getDirectiveValues(
//       GraphQLIncludeDirective,
//       node,
//       variableValues,
//     );
//     if (include?.if === false) {
//       return false;
//     }
//     return true;
//   }

#[cfg(test)]
mod tests {
    use std::fs;

    use super::build_operation;

    #[test]
    fn it_works() {
        let contents = fs::read_to_string("src/tests/github_query.graphql");
        let v = contents.unwrap();
        build_operation(v.as_str());
    }
}

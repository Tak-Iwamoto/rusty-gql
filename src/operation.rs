use std::collections::{HashMap, HashSet};

use anyhow::Result;
use graphql_parser::query::{Definition, Field, FragmentDefinition, Selection, SelectionSet};

pub enum GraphQLOperationType {
    Query,
    Mutation,
    Subscription,
}

// single or multiのハンドリングをする
pub struct GraphQLOperation<'a> {
    pub operation_type: GraphQLOperationType,
    pub operation_name: Option<String>,
    pub fragments: HashMap<String, FragmentDefinition<'a, &'a str>>,
    pub definitions: Vec<Definition<'a, &'a str>>,
    // pub variables:
    // pub errors
}

//graphql-jsのbuildExecutionContextが参考になりそう
// fragmentとoperationと収集したfieldsを返すように実装する
pub fn build_operation(
    query_doc: &str,
    operation_name: Option<String>,
) -> Result<GraphQLOperation> {
    let mut fragments = HashMap::new();
    let parsed_query = graphql_parser::parse_query::<&str>(query_doc)?;

    // TODO: multiple operationに対応する
    let first_def = &parsed_query.definitions[0];

    let operation_type = match first_def {
        Definition::Operation(operation) => match operation {
            graphql_parser::query::OperationDefinition::SelectionSet(_) => unreachable!(),
            graphql_parser::query::OperationDefinition::Query(_) => GraphQLOperationType::Query,
            graphql_parser::query::OperationDefinition::Mutation(_) => {
                GraphQLOperationType::Mutation
            }
            graphql_parser::query::OperationDefinition::Subscription(_) => {
                GraphQLOperationType::Subscription
            }
        },
        Definition::Fragment(_) => unreachable!(),
    };

    let definitions = parsed_query.definitions;
    for definition in &definitions {
        match definition {
            graphql_parser::query::Definition::Operation(_) => continue,
            graphql_parser::query::Definition::Fragment(fragment) => {
                let name = fragment.name.to_string();
                fragments.insert(name, fragment.to_owned());
            }
        }
    }

    Ok(GraphQLOperation {
        operation_type,
        operation_name,
        definitions,
        fragments,
    })
}

// pub fn collect_fields<'a>(
//     fragments: HashMap<String, FragmentDefinition<'a, &'a str>>,
//     selection_set: SelectionSet<'a, &'a str>,
// ) -> HashMap<String, Vec<Field<'a, &'a str>>> {
//     let mut fields_map: HashMap<String, Vec<Field<&str>>> = HashMap::new();
//     let mut visited_fragments = HashSet::new();
//     for item in selection_set.items {
//         match item {
//             Selection::Field(field) => match fields_map.get(&field.name.to_string()) {
//                 Some(_) => {
//                     fields_map
//                         .get_mut(&field.name.to_string())
//                         .unwrap()
//                         .push(field);
//                 }
//                 None => {
//                     fields_map.insert(field.name.to_string(), vec![field]);
//                 }
//             },
//             Selection::FragmentSpread(spread_frg) => {
//                 let fragment_name = spread_frg.fragment_name;
//                 if visited_fragments.contains(fragment_name) {
//                     continue;
//                 }
//                 visited_fragments.insert(fragment_name);
//                 let fragment = fragments.get(fragment_name);
//                 match fragment {
//                     Some(frg) => return collect_fields(fragments, frg.selection_set),
//                     None => continue,
//                 }
//             }
//             Selection::InlineFragment(inline_frg) => {
//                 collect_fields(fragments, inline_frg.selection_set);
//             }
//         }
//     }
//     fields_map
// }

#[cfg(test)]
mod tests {
    use std::fs;

    use super::build_operation;

    #[test]
    fn it_works() {
        let contents = fs::read_to_string("src/tests/multiple_operation.graphql");
        let v = contents.unwrap();
        build_operation(v.as_str(), None);
    }
}

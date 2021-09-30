use std::fs;

use anyhow::Result;
use graphql_parser::{query::SelectionSet, schema::Text};

fn test() {
    let contents = fs::read_to_string("src/tests/github_query.graphql");
    let v = contents.unwrap();
    parse_query(v.as_str());
}

//graphql-jsのbuildExecutionContextが参考になりそう
fn parse_query(query_doc: &str) -> Result<()> {
    let parsed_query = graphql_parser::parse_query::<&str>(query_doc)?;
    for node in parsed_query.definitions {
        match node {
            graphql_parser::query::Definition::Operation(operation) => match operation {
                graphql_parser::query::OperationDefinition::SelectionSet(selection) => {
                    parse_selection_set(selection)
                }
                graphql_parser::query::OperationDefinition::Query(query) => {
                    for item in query.selection_set.items {
                        match item {
                            graphql_parser::query::Selection::Field(field) => {
                                parse_selection_set(field.selection_set)
                            }
                            graphql_parser::query::Selection::FragmentSpread(spread) => {
                                println!("{:?}", spread.fragment_name)
                            }
                            graphql_parser::query::Selection::InlineFragment(inline_fragment) => {
                                println!("{:?}", inline_fragment.selection_set)
                            }
                        }
                    }
                }
                graphql_parser::query::OperationDefinition::Mutation(mutation) => {
                    println!("mutation");
                    println!("{:?}", mutation.name)
                }
                graphql_parser::query::OperationDefinition::Subscription(sub) => {
                    println!("sub");
                    println!("{:?}", sub.name)
                }
            },
            graphql_parser::query::Definition::Fragment(fragment) => {
                println!("sub");
                println!("{:?}", fragment.name)
            }
        }
    }
    Ok(())
}

fn parse_selection_set<'a, T: Text<'a>>(selection_set: SelectionSet<'a, T>) {
    for item in selection_set.items {
        match item {
            graphql_parser::query::Selection::Field(field) => {
                println!("{:?}", field.name);
                parse_selection_set(field.selection_set);
            }
            graphql_parser::query::Selection::FragmentSpread(_) => todo!(),
            graphql_parser::query::Selection::InlineFragment(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test;

    #[test]
    fn it_works() {
        test()
    }
}

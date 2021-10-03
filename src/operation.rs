use anyhow::Result;
use graphql_parser::{query::SelectionSet, schema::Text};

enum GraphQLOperation {
    Query,
    Mutation,
    Subscription,
}

//graphql-jsのbuildExecutionContextが参考になりそう
fn build_operation(doc: &str) -> Result<()> {
    let parsed_query = graphql_parser::parse_query::<&str>(doc)?;
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
                            graphql_parser::query::Selection::FragmentSpread(spread) => {}
                            graphql_parser::query::Selection::InlineFragment(inline_fragment) => {}
                        }
                    }
                }
                graphql_parser::query::OperationDefinition::Mutation(mutation) => {}
                graphql_parser::query::OperationDefinition::Subscription(sub) => {}
            },
            graphql_parser::query::Definition::Fragment(fragment) => {}
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
    use std::fs;

    use super::build_operation;

    #[test]
    fn it_works() {
        let contents = fs::read_to_string("src/tests/github_query.graphql");
        let v = contents.unwrap();
        build_operation(v.as_str());
    }
}

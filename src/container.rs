use std::{collections::HashMap, pin::Pin};

use futures::future::BoxFuture;

use crate::{graphql_value::GraphQLValue, GraphQLResponse};

// Container holds the information of the resolver
// なぜBoxFuture型にするかというとtraitで非同期関数を定義していて、そこの戻り値はBox dyn Futureを返すため、BoxFutureを使用している
type GraphQLFuture<'a> = BoxFuture<'a, GraphQLResponse<GraphQLValue>>;
// type BoxFieldFuture<'a> = Pin<Box<dyn Future<Output = ServerResult<(Name, Value)>> + 'a + Send>>;

pub struct Container<'a> {
    // key is parent_type and target field
    // value is resolve fn
    resolvers: HashMap<(&'a str, &'a str), GraphQLFuture<'a>>,
}

async fn test_async() -> GraphQLResponse<GraphQLValue> {
    Ok(GraphQLValue::Null)
    // Box::pin(async move { Ok(GraphQLValue::Null) })
}

async fn test_async1() -> GraphQLResponse<GraphQLValue> {
    Ok(GraphQLValue::Int(1))
}

// hashmapから取り出したresolverが&Pin型になっているので値が取り出せないが、async-graphqlではPinなのでjoin_allできている
// hashmapからgetした場合は所有権がhashmapにあるので、参照としてしか取り出すことができない
// removeすればhashmapから所有権がなくなるので、値として取得することができる
// hashmapにresolver関数を格納するのは無理そう
fn build_resolvers<'a>() -> HashMap<
    (&'a str, &'a str),
    Pin<Box<impl futures::Future<Output = GraphQLResponse<GraphQLValue>>>>,
> {
    let mut future_map = HashMap::new();
    future_map.insert(("query", "show"), Box::pin(test_async()));
    // future_map.insert(("query", "test"), Box::pin(test_async1()));
    future_map
}

#[cfg(test)]
mod tests {
    use futures::join;

    use crate::container::build_resolvers;
    use crate::graphql_value::GraphQLValue;
    use crate::GraphQLResponse;

    use super::test_async;
    use super::test_async1;

    #[tokio::test]
    async fn it_works() {
        let mut resolvers = build_resolvers();
        let resolver = &resolvers.get(&("query", "show")).unwrap();

        // assert_eq!(value, GraphQLValue::Null);
    }
}

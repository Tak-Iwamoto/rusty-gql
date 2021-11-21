use rusty_gql::async_trait::async_trait;
use rusty_gql::{GqlValue, Object, Resolver};

pub struct Query;

#[derive(Debug)]
pub struct Show {
    name: String,
    description: String,
}

#[async_trait]
impl Resolver for Show {
    async fn resolve(
        &self,
        context: &rusty_gql::ExecutionContext,
    ) -> rusty_gql::Response<GqlValue> {
        let value = GqlValue::Null;
        Ok(value)
    }
}

#[Object]
impl Query {
    async fn test(&self) -> Show {
        let show = Show {
            name: "test".to_string(),
            description: "test".to_string(),
        };
        show
    }

    async fn result_test(&self) -> Result<Show, String> {
        let show = Show {
            name: "test".to_string(),
            description: "test".to_string(),
        };
        Ok(show)
    }
}

#[tokio::test]
async fn it_works() {
    let query = Query {};
    let value = query.result_test().await;
}

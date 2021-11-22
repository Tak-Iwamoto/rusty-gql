use rusty_gql::async_trait::async_trait;
use rusty_gql::{ExecutionContext, Resolver, GqlValue, Object};

pub struct Query;

#[derive(Debug)]
pub struct Show {
    name: String,
    description: String,
}

#[async_trait]
impl Resolver for Show {
    async fn resolve(&self, ctx: &ExecutionContext) -> rusty_gql::Response<Option<GqlValue>> {
        Ok(Some(GqlValue::Null))
    }
}

#[Object]
impl Query {
    pub async fn get_shows(&self) -> Show {
        let show = Show {
            name: "test".to_string(),
            description: "test".to_string(),
        };
        show
    }

    pub async fn get_show2<'a>(&self, ctx: &ExecutionContext<'a>) -> Result<Show, String> {
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
    // let value = query.result_test().await;
}

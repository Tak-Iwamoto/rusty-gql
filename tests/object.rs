use rusty_gql::async_trait::async_trait;
use rusty_gql::{FieldContext, GqlValue, Resolver, SelectionSetContext, SelectionSetResolver};

pub struct Query;

#[derive(Debug)]
pub struct Show {
    name: String,
    description: String,
}

#[async_trait]
impl Resolver for Show {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> rusty_gql::Response<Option<GqlValue>> {
        // resolve_object(self, ctx, true).await.map(Some)
        Ok(Some(GqlValue::Null))
    }
}

#[async_trait]
impl SelectionSetResolver for Show {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> rusty_gql::Response<GqlValue> {
        Ok(GqlValue::Null)
    }
}

// #[Object]
impl Query {
    pub async fn get_shows(&self, ctx: &FieldContext<'_>) -> Show {
        let show = Show {
            name: "test".to_string(),
            description: "test".to_string(),
        };
        show
    }

    pub async fn get_show2(&self, ctx: &FieldContext<'_>) -> Show {
        let show = Show {
            name: "test".to_string(),
            description: "test".to_string(),
        };
        show
    }
}

#[async_trait]
impl Resolver for Query {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> rusty_gql::Response<Option<GqlValue>> {
        if ctx.item.name == "get_shows" {
            let resolve_fn = async move { self.get_shows(ctx).await };

            let obj = resolve_fn.await;
            let selection_set = ctx.with_selection_set(&ctx.item.selection_set);
            return selection_set.resolve_selection(&obj, true).await.map(Some);
        }
        Ok(None)
    }
}

#[async_trait]
impl SelectionSetResolver for Query {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> rusty_gql::Response<GqlValue> {
        ctx.resolve_selection(self, true).await
    }
}

#[tokio::test]
async fn it_works() {
    let query = Query {};
    // let value = query.result_test().await;
}

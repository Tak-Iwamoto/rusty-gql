use rusty_gql::async_trait::async_trait;
use rusty_gql::{
    FieldContext, GqlResolver, GqlValue, Resolver, SelectionSetContext, SelectionSetResolver,
};

pub struct Query;

#[derive(Debug)]
pub struct Show {
    pub name: String,
    pub description: String,
}

#[async_trait]
impl Resolver for Show {
    async fn resolve_field(
        &self,
        _ctx: &FieldContext<'_>,
    ) -> rusty_gql::ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Null))
    }
}

#[async_trait]
impl SelectionSetResolver for Show {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> rusty_gql::ResolverResult<GqlValue> {
        Ok(GqlValue::Null)
    }
}

#[GqlResolver]
impl Query {
    pub async fn get_shows(&self) -> Show {
        let show = Show {
            name: "test".to_string(),
            description: "test".to_string(),
        };
        show
    }

    pub async fn get_show2(&self) -> Show {
        let show = Show {
            name: "test".to_string(),
            description: "test".to_string(),
        };
        show
    }
}

#[tokio::test]
async fn it_works() {
    let query = Query {};
    // let value = query.result_test().await;
}

use crate::graphql::*;
use rusty_gql::*;

pub struct Droid {
    id: ID,
    name: String,
    primaryFunction: Option<String>,
}

#[Resolver]
impl Droid {
    async fn id(&self) -> ID {
        self.id.clone()
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn friends(&self, first: Option<i64>, after: Option<ID>) -> FriendsConnection {
        todo!()
    }

    async fn appearsIn(&self) -> Vec<Option<Episode>> {
        todo!()
    }

    async fn primaryFunction(&self) -> Option<String> {
        self.primaryFunction.clone()
    }
}
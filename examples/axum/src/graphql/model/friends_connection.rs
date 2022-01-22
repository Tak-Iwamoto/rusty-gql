use crate::graphql::*;
use rusty_gql::*;

pub struct FriendsConnection {
    totalCount: Option<i64>,
}

#[Resolver]
impl FriendsConnection {
    async fn totalCount(&self) -> Option<i64> {
        self.totalCount
    }

    async fn edges(&self) -> Option<Vec<Option<FriendsEdge>>> {
        todo!()
    }

    async fn pageInfo(&self) -> PageInfo {
        todo!()
    }
}
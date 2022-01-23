use crate::graphql::*;
use rusty_gql::*;

pub struct FriendsConnection {
    pub totalCount: Option<i64>,
}

#[Resolver]
impl FriendsConnection {
    pub async fn totalCount(&self) -> Option<i64> {
        self.totalCount
    }

    pub async fn edges(&self) -> Option<Vec<Option<FriendsEdge>>> {
        todo!()
    }

    pub async fn pageInfo(&self) -> PageInfo {
        todo!()
    }
}
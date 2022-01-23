use crate::graphql::*;
use rusty_gql::*;

pub struct FriendsConnection {
    pub totalCount: Option<i64>,
    pub edges: Vec<FriendsEdge>,
    pub pageInfo: PageInfo,
}

#[GqlType]
impl FriendsConnection {
    pub async fn totalCount(&self) -> Option<i64> {
        self.totalCount
    }

    pub async fn edges(&self) -> Vec<FriendsEdge> {
        self.edges.clone()
    }

    pub async fn pageInfo(&self) -> PageInfo {
        self.pageInfo.clone()
    }
}

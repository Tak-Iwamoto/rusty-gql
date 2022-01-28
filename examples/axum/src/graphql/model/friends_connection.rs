#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

pub struct FriendsConnection {
    pub totalCount: Option<i32>,
    pub edges: Vec<FriendsEdge>,
    pub pageInfo: PageInfo,
}

#[GqlType]
impl FriendsConnection {
    pub async fn totalCount(&self, ctx: &Context<'_>) -> Option<i32> {
        self.totalCount
    }

    pub async fn edges(&self, ctx: &Context<'_>) -> Vec<FriendsEdge> {
        self.edges.clone()
    }

    pub async fn pageInfo(&self, ctx: &Context<'_>) -> PageInfo {
        self.pageInfo.clone()
    }
}

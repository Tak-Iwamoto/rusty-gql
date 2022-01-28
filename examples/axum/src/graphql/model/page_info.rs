#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(Clone)]
pub struct PageInfo {
    pub startCursor: Option<ID>,
    pub endCursor: Option<ID>,
    pub hasPreviousPage: bool,
    pub hasNextPage: bool,
}

#[GqlType]
impl PageInfo {
    pub async fn startCursor(&self, ctx: &Context<'_>) -> Option<ID> {
        self.startCursor.clone()
    }

    pub async fn endCursor(&self, ctx: &Context<'_>) -> Option<ID> {
        self.endCursor.clone()
    }

    pub async fn hasPreviousPage(&self, ctx: &Context<'_>) -> bool {
        self.hasPreviousPage
    }

    pub async fn hasNextPage(&self, ctx: &Context<'_>) -> bool {
        self.hasNextPage
    }
}

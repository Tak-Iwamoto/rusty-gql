use crate::graphql::*;
use rusty_gql::*;

pub struct PageInfo {
    startCursor: Option<ID>,
    endCursor: Option<ID>,
    hasPreviousPage: bool,
    hasNextPage: bool,
}

#[Resolver]
impl PageInfo {
    async fn startCursor(&self) -> Option<ID> {
        self.startCursor.clone()
    }

    async fn endCursor(&self) -> Option<ID> {
        self.endCursor.clone()
    }

    async fn hasPreviousPage(&self) -> bool {
        self.hasPreviousPage
    }

    async fn hasNextPage(&self) -> bool {
        self.hasNextPage
    }
}
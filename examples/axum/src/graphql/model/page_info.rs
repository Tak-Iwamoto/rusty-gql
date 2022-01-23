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
    pub async fn startCursor(&self) -> Option<ID> {
        self.startCursor.clone()
    }

    pub async fn endCursor(&self) -> Option<ID> {
        self.endCursor.clone()
    }

    pub async fn hasPreviousPage(&self) -> bool {
        self.hasPreviousPage
    }

    pub async fn hasNextPage(&self) -> bool {
        self.hasNextPage
    }
}

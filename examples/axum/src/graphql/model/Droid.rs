use crate::graphql::*;
use rusty_gql::*;

pub struct Droid {
    pub id: ID,
    pub name: String,
    pub primaryFunction: Option<String>,
}

#[Resolver]
impl Droid {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }

    pub async fn name(&self) -> String {
        self.name.clone()
    }

    pub async fn friends(&self, first: Option<i64>, after: Option<ID>) -> FriendsConnection {
        todo!()
    }

    pub async fn appearsIn(&self) -> Vec<Option<Episode>> {
        todo!()
    }

    pub async fn primaryFunction(&self) -> Option<String> {
        self.primaryFunction.clone()
    }
}
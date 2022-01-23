use crate::graphql::*;
use rusty_gql::*;

pub struct FriendsEdge {
    pub cursor: ID,
}

#[Resolver]
impl FriendsEdge {
    pub async fn cursor(&self) -> ID {
        self.cursor.clone()
    }

    pub async fn node(&self) -> Option<Character> {
        todo!()
    }
}
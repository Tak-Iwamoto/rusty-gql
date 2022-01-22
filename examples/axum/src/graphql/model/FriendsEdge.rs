use crate::graphql::*;
use rusty_gql::*;

pub struct FriendsEdge {
    cursor: ID,
}

#[Resolver]
impl FriendsEdge {
    async fn cursor(&self) -> ID {
        self.cursor.clone()
    }

    async fn node(&self) -> Option<Character> {
        todo!()
    }
}
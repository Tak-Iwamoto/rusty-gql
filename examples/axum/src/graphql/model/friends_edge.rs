use crate::graphql::*;
use rusty_gql::*;

#[derive(Clone)]
pub struct FriendsEdge {
    pub cursor: ID,
    pub node: Option<Character>
}

#[Resolver]
impl FriendsEdge {
    pub async fn cursor(&self) -> ID {
        self.cursor.clone()
    }

    pub async fn node(&self) -> Option<Character> {
        self.node.clone()
    }
}

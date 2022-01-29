#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(Clone)]
pub struct FriendsEdge {
    pub cursor: ID,
    pub node: Option<Character>,
}

#[GqlType]
impl FriendsEdge {
    pub async fn cursor(&self, ctx: &Context<'_>) -> ID {
        self.cursor.clone()
    }

    pub async fn node(&self, ctx: &Context<'_>) -> Option<Character> {
        self.node.clone()
    }
}

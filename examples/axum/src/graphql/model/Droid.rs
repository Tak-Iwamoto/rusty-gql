#![allow(warnings, unused)]
use crate::{
    graphql::*,
    starwars::{han, luke},
};
use rusty_gql::*;

#[derive(Debug, Clone)]
pub struct Droid {
    pub id: ID,
    pub name: String,
    pub primaryFunction: Option<String>,
}

#[GqlType]
impl Droid {
    pub async fn id(&self, ctx: &Context<'_>) -> ID {
        self.id.clone()
    }

    pub async fn name(&self, ctx: &Context<'_>) -> String {
        self.name.clone()
    }

    pub async fn friends(&self, ctx: &Context<'_>,first: Option<i32>, after: Option<ID>) -> FriendsConnection {
        FriendsConnection {
            totalCount: Some(4),
            edges: vec![
                FriendsEdge {
                    cursor: ID::from("1"),
                    node: Some(Character::Human(luke())),
                },
                FriendsEdge {
                    cursor: ID::from("3"),
                    node: Some(Character::Human(han())),
                },
            ],
            pageInfo: PageInfo {
                startCursor: None,
                endCursor: None,
                hasPreviousPage: false,
                hasNextPage: false,
            },
        }
    }

    pub async fn appearsIn(&self, ctx: &Context<'_>) -> Vec<Episode> {
        if self.name == "R2D2".to_string() {
            vec![Episode::EMPIRE, Episode::NEWHOPE, Episode::JEDI]
        } else if self.name == "C3PO".to_string() {
            vec![Episode::EMPIRE, Episode::NEWHOPE]
        } else {
            vec![]
        }
    }

    pub async fn primaryFunction(&self, ctx: &Context<'_>) -> Option<String> {
        self.primaryFunction.clone()
    }
}

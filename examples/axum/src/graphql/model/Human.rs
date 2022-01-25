use crate::{
    graphql::*,
    starwars::{han, luke},
};
use rusty_gql::*;

#[derive(Debug, Clone)]
pub struct Human {
    pub id: ID,
    pub name: String,
    pub homePlanet: Option<String>,
    pub height: Option<f64>,
    pub mass: Option<f64>,
}

#[GqlType]
impl Human {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }

    pub async fn name(&self) -> String {
        self.name.clone()
    }

    pub async fn homePlanet(&self) -> Option<String> {
        self.homePlanet.clone()
    }

    pub async fn height(&self, unit: Option<LengthUnit>) -> Option<f64> {
        self.height
    }

    pub async fn mass(&self) -> Option<f64> {
        self.mass
    }

    pub async fn episode(&self) -> Option<Episode> {
        Some(Episode::JEDI)
    }

    pub async fn friends(&self, first: Option<i64>, after: Option<ID>) -> FriendsConnection {
        if self.id.0 == "2".to_string() {
            FriendsConnection {
                totalCount: Some(0),
                edges: vec![],
                pageInfo: PageInfo {
                    startCursor: None,
                    endCursor: None,
                    hasPreviousPage: false,
                    hasNextPage: false,
                },
            }
        } else {
            FriendsConnection {
                totalCount: Some(2),
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
    }

    pub async fn appearsIn(&self) -> Vec<Episode> {
        vec![Episode::NEWHOPE, Episode::JEDI, Episode::EMPIRE]
    }
}

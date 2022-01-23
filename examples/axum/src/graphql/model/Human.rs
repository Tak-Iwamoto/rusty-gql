use crate::graphql::*;
use rusty_gql::*;

#[derive(Debug, Clone)]
pub struct Human {
    pub id: ID,
    pub name: String,
    pub homePlanet: Option<String>,
    pub height: Option<f64>,
    pub mass: Option<f64>,
}

#[Resolver]
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
        todo!()
    }

    pub async fn friends(&self, first: Option<i64>, after: Option<ID>) -> FriendsConnection {
        todo!()
    }

    pub async fn appearsIn(&self) -> Vec<Option<Episode>> {
        todo!()
    }
}

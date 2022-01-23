use crate::graphql::*;
use rusty_gql::*;

pub struct Human {
    pub id: ID,
    pub name: String,
    pub homePlanet: Option<String>,
    pub height: Option<f64>,
    pub mass: Option<f64>,
}

#[Resolver]
impl Human {
    async fn id(&self) -> ID {
        self.id.clone()
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn homePlanet(&self) -> Option<String> {
        self.homePlanet.clone()
    }

    async fn height(&self, unit: Option<LengthUnit>) -> Option<f64> {
        self.height
    }

    async fn mass(&self) -> Option<f64> {
        self.mass
    }

    async fn episode(&self) -> Option<Episode> {
        todo!()
    }

    async fn friends(&self, first: Option<i64>, after: Option<ID>) -> FriendsConnection {
        todo!()
    }

    async fn appearsIn(&self) -> Vec<Option<Episode>> {
        todo!()
    }
}

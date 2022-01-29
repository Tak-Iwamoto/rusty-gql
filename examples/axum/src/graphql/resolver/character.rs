#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlInterface, Debug, Clone)]
pub enum Character {
    Human(Human),
    Droid(Droid),
}

#[GqlType(interface)]
impl Character {
    async fn id(&self, ctx: &Context<'_>) -> Result<ID, Error> {
        match self {
            Character::Human(obj) => obj.id(&ctx).await,
            Character::Droid(obj) => obj.id(&ctx).await,
        }
    }

    async fn name(&self, ctx: &Context<'_>) -> Result<String, Error> {
        match self {
            Character::Human(obj) => obj.name(&ctx).await,
            Character::Droid(obj) => obj.name(&ctx).await,
        }
    }

    async fn friends(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        after: Option<ID>,
    ) -> Result<FriendsConnection, Error> {
        match self {
            Character::Human(obj) => obj.friends(&ctx, first, after).await,
            Character::Droid(obj) => obj.friends(&ctx, first, after).await,
        }
    }

    async fn appearsIn(&self, ctx: &Context<'_>) -> Result<Vec<Episode>, Error> {
        match self {
            Character::Human(obj) => obj.appearsIn(&ctx).await,
            Character::Droid(obj) => obj.appearsIn(&ctx).await,
        }
    }
}

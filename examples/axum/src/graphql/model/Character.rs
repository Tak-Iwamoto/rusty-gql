use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlInterface, Debug, Clone)]
pub enum Character {
    Human(Human),
    Droid(Droid),
}

#[GqlType(interface)]
impl Character {
    async fn id(&self, ctx: &FieldContext<'_>) -> Result<ID, Error> {
        match self {
            Character::Human(obj) => Ok(obj.id(&ctx).await?),
            Character::Droid(obj) => Ok(obj.id(&ctx).await?),
        }
    }

    async fn name(&self, ctx: &FieldContext<'_>) -> Result<String, Error> {
        match self {
            Character::Human(obj) => Ok(obj.name(&ctx).await?),
            Character::Droid(obj) => Ok(obj.name(&ctx).await?),
        }
    }

    async fn friends(
        &self,
        ctx: &FieldContext<'_>,
        first: Option<i64>,
        after: Option<ID>,
    ) -> Result<FriendsConnection, Error> {
        match self {
            Character::Human(obj) => Ok(obj.friends(&ctx, first, after).await?),
            Character::Droid(obj) => Ok(obj.friends(&ctx, first, after).await?),
        }
    }

    async fn appearsIn(&self, ctx: &FieldContext<'_>) -> Result<Vec<Episode>, Error> {
        match self {
            Character::Human(obj) => Ok(obj.appearsIn(&ctx).await?),
            Character::Droid(obj) => Ok(obj.appearsIn(&ctx).await?),
        }
    }
}

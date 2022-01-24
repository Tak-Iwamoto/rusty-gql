use crate::graphql::*;
use rusty_gql::*;

#[derive(GqlInterface, Debug, Clone)]
pub enum Character {
    Human(Human),
    Droid(Droid),
}

#[GqlType(interface)]
impl Character {
    async fn name(&self) -> String {
        match self {
            Character::Human(obj) => obj.name.clone(),
            Character::Droid(obj) => obj.name.clone(),
        }
    }
}

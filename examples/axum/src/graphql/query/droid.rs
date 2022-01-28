#![allow(warnings, unused)]
use crate::{
    graphql::*,
    starwars::{c3po, r2d2},
};
use rusty_gql::*;

pub async fn droid(ctx: &Context<'_>, id: ID) -> Option<Droid> {
    if id.0 == "5" {
        Some(r2d2())
    } else if id.0 == "6" {
        Some(c3po())
    } else {
        ctx.add_error(&GqlError::new("Droid Not found", None));
        None
    }
}

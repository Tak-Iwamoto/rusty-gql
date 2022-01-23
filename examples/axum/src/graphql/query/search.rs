use crate::{
    graphql::*,
    starwars::{c3po, han, leia, luke, r2d2, vader},
};
use rusty_gql::*;

pub async fn search(text: Option<String>, episode: Option<Episode>) -> Vec<SearchResult> {
    if let Some(text) = text {
        if text == "luke" {
            vec![SearchResult::Human(luke())]
        } else if text == "vader" {
            vec![SearchResult::Human(vader())]
        } else if text == "han" {
            vec![SearchResult::Human(han())]
        } else if text == "leia" {
            vec![SearchResult::Human(leia())]
        } else if text == "r2d2" {
            vec![SearchResult::Droid(r2d2())]
        } else if text == "c3po" {
            vec![SearchResult::Droid(c3po())]
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

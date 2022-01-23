use rusty_gql::ID;

use crate::graphql::{Droid, Episode, Human, Review};

pub fn luke() -> Human {
    Human {
        id: ID("1".to_string()),
        name: "Luke Skywalker".to_string(),
        homePlanet: Some("Tatooine".to_string()),
        height: Some(180.0),
        mass: Some(70.0),
    }
}

pub fn vader() -> Human {
    Human {
        id: ID("2".to_string()),
        name: "Anakin Skywalker".to_string(),
        homePlanet: Some("Tatooine".to_string()),
        height: Some(190.0),
        mass: Some(80.0),
    }
}

pub fn han() -> Human {
    Human {
        id: ID("3".to_string()),
        name: "Han Solo".to_string(),
        homePlanet: None,
        height: Some(175.0),
        mass: Some(70.0),
    }
}

pub fn leia() -> Human {
    Human {
        id: ID("4".to_string()),
        name: "Leia Organa".to_string(),
        homePlanet: None,
        height: None,
        mass: None,
    }
}

pub fn r2d2() -> Droid {
    Droid {
        id: ID("5".to_string()),
        name: "R2D2".to_string(),
        primaryFunction: Some("support jedi".to_string()),
    }
}

pub fn c3po() -> Droid {
    Droid {
        id: ID("6".to_string()),
        name: "C3PO".to_string(),
        primaryFunction: Some("communication".to_string()),
    }
}

pub fn all_reviews() -> Vec<Review> {
    vec![
        Review {
            stars: 3,
            commentary: None,
            episode: Some(Episode::EMPIRE),
        },
        Review {
            stars: 5,
            commentary: Some("Great!".to_string()),
            episode: Some(Episode::NEWHOPE),
        },
        Review {
            stars: 4,
            commentary: None,
            episode: Some(Episode::JEDI),
        },
    ]
}

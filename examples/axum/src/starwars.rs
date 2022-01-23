use rusty_gql::ID;

use crate::graphql::Human;

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

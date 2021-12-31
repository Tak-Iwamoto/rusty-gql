pub fn main_file_content() -> &'static str {
    r#"fn main() {
    println!("Hello, world!");
}"#
}

pub fn cargo_toml_content(app_name: &str) -> String {
    r#"[package]
name = "APP_NAME"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = {version = "0.4.2", features = ["headers"]}
hyper = "0.14.16"
rusty-gql = "^1"
rusty-gql-axum = {path = "../../frameworks/axum"}
tokio = {version = "1.12.0", features = ["full"]}"#
        .replace("APP_NAME", app_name)
}

pub fn starwars_schema() -> &'static str {
    r#"
        schema {
            query: Query
            mutation: Mutation
            subscription: Subscription
        }
        type Query {
            hero(episode: Episode): Character
            reviews(episode: Episode!): [Review]
            search(text: String): [SearchResult]
            character(id: ID!): Character
            droid(id: ID!): Droid
            human(id: ID!): Human
            starship(id: ID!): Starship
        }

        type Mutation {
            createReview(episode: Episode, review: ReviewInput!): Review
        }

        type Subscription {
            reviewAdded(episode: Episode): Review
        }
        enum Episode {
            NEWHOPE
            EMPIRE
            JEDI
        }

        interface Character {
            id: ID!
            name: String!
            friends: [Character]
            friendsConnection(first: Int, after: ID): FriendsConnection!
            appearsIn: [Episode]!
        }

        enum LengthUnit {
            METER
            FOOT
        }

        type Human implements Character {
            id: ID!
            name: String!
            homePlanet: String
            height(unit: LengthUnit = METER): Float
            mass: Float
            friends: [Character]
            friendsConnection(first: Int, after: ID): FriendsConnection!
            appearsIn: [Episode]!
            starships: [Starship]
        }

        type Droid implements Character {
            id: ID!
            name: String!
            friends: [Character]
            friendsConnection(first: Int, after: ID): FriendsConnection!
            appearsIn: [Episode]!
            primaryFunction: String
        }

        type FriendsConnection {
            totalCount: Int
            edges: [FriendsEdge]
            friends: [Character]
            pageInfo: PageInfo!
        }

        type FriendsEdge {
            cursor: ID!
            node: Character
        }

        type PageInfo {
            startCursor: ID
            endCursor: ID
            hasNextPage: Boolean!
        }

        type Review {
            episode: Episode
            stars: Int!
            commentary: String
        }

        input ReviewInput {
            stars: Int!
            commentary: String
            favorite_color: ColorInput
        }

        input ColorInput {
            red: Int!
            green: Int!
            blue: Int!
        }

        type Starship {
            id: ID!
            name: String!
            length(unit: LengthUnit = METER): Float
            coordinates: [[Float!]!]
        }

        union SearchResult = Human | Droid | Starship
    "#
}

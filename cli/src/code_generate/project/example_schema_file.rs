use crate::code_generate::FileDefinition;

pub struct TodoSchemaFile<'a> {
    pub app_name: &'a str,
}
impl<'a> FileDefinition for TodoSchemaFile<'a> {
    fn name(&self) -> String {
        "schema.graphql".to_string()
    }

    fn path(&self) -> String {
        format!("{}/schema/schema.graphql", self.app_name)
    }

    fn content(&self) -> String {
        todo_schema_content().to_string()
    }
}

fn todo_schema_content() -> &'static str {
    r#"type Query {
  todos(first: Int): [Todo!]!
}
type Todo {
  title: String!
  content: String
  done: Boolean!
}
"#
}

pub struct StarWarsSchemaFile<'a> {
    pub app_name: &'a str,
}

impl<'a> FileDefinition for StarWarsSchemaFile<'a> {
    fn name(&self) -> String {
        "schema.graphql".to_string()
    }

    fn path(&self) -> String {
        format!("{}/schema/schema.graphql", self.app_name)
    }

    fn content(&self) -> String {
        starwars_schema_content().to_string()
    }
}

fn starwars_schema_content() -> &'static str {
    r#"
type Query {
  hero(episode: Episode): Character
  reviews(episode: Episode!): [Review!]!
  search(text: String, episode: Episode): [SearchResult!]!
  character(id: ID!): Character
  droid(id: ID!): Droid
  human(id: ID!): Human
}

type Mutation {
  createReview(episode: Episode, review: ReviewInput!): Review
}

enum Episode {
  NEWHOPE
  EMPIRE
  JEDI
}

interface Character {
  id: ID!
  name: String!
  friends(first: Int, after: ID): FriendsConnection!
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
  episode: Episode
  friends(first: Int, after: ID): FriendsConnection!
  appearsIn: [Episode!]!
}

type Droid implements Character {
  id: ID!
  name: String!
  friends(first: Int, after: ID): FriendsConnection!
  appearsIn: [Episode!]!
  primaryFunction: String
}

type FriendsConnection {
  totalCount: Int
  edges: [FriendsEdge!]!
  pageInfo: PageInfo!
}

type FriendsEdge {
  cursor: ID!
  node: Character
}

type PageInfo {
  startCursor: ID
  endCursor: ID
  hasPreviousPage: Boolean!
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
}

union SearchResult = Human | Droid

directive @possibleTypes(
  abstractType: String
  concreteTypes: [String!]!
) on INPUT_FIELD_DEFINITION

scalar DateTime
"#
}

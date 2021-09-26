use askama::Template;

#[derive(Template)]
#[template(path = "graphiql.html")]
pub struct GraphiQLTemplate {}

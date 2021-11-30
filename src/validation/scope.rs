#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope<'a> {
    Operation(Option<&'a str>),
    Fragment(&'a str),
}

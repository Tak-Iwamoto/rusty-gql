mod request;
mod response;

pub use request::GqlRequest;
pub use response::GqlResponse;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

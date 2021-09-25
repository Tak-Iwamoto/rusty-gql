use async_trait::async_trait;

#[async_trait]
pub trait Resolver {
    async fn resolve(&self) {}
}

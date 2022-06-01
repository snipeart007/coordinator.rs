use async_trait::async_trait;

#[async_trait]
pub trait SpaceDataInterface<T, const K: usize> {
    type Error: std::error::Error;

    async fn insert(&mut self, id: String, position: &[T; K]) -> Result<(), Self::Error>;
    async fn remove(&mut self, id: String) -> Result<(), Self::Error>;
    // async fn subscribe(&mut self, id: String) -> Result<(), Self::Error>;
}

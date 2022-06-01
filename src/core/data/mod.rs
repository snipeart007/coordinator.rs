use async_trait::async_trait;
use kiddo::KdTree;
use scc::HashMap;

use self::error::SpaceDataError;

mod error;
mod interface;

pub struct SpaceDataService {
    tree: KdTree<f32, String, 3>,
    map: HashMap<String, [f32; 3]>,
}

#[async_trait]
impl interface::SpaceDataInterface<f32, 3> for SpaceDataService {
    type Error = error::SpaceDataError;
    // pub fn new() -> Self {
    //     let tree: KdTree<f32, String, 3> = KdTree::new();
    //     Self { tree }
    // }

    async fn insert(&mut self, id: String, position: &[f32; 3]) -> Result<(), Self::Error> {
        self.tree.add(position, id.to_owned())?;
        let result = self.map.insert_async(id.to_owned(), position.to_owned()).await;
        match result {
            Ok(()) => return Ok(()),
            Err(_) => {
                return Err(SpaceDataError::DataError(error::DataError::EntityAlreadyExists(id)));
            },
        }
    }

    async fn remove(&mut self, id: String) -> Result<(), Self::Error> {
        let position = self
            .map
            .read_async(&id, |_, v| *v)
            .await
            .ok_or(error::DataError::EntityNotFound(id.clone()))?;

        self.tree.remove(&position, &id)?;
        Ok(())
    }
}

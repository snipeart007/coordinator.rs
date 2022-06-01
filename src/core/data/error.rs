use thiserror::Error;

#[derive(Debug, Error)]
pub enum HashMapError {}

#[derive(Debug, Error)]
pub enum DataError {
    #[error("no entity found with id `{0}`")]
    EntityNotFound(String),
    #[error("existing entity found with id `{0}`")]
    EntityAlreadyExists(String)
}

#[derive(Error, Debug)]
pub enum SpaceDataError {
    #[error("kdtree error, from kiddo crate")]
    KdTreeError(#[from] kiddo::ErrorKind),
    #[error("hashmap error, from scc crate")]
    HashMapError(#[from] HashMapError),
    #[error("error from space data itself")]
    DataError(#[from] DataError),
}

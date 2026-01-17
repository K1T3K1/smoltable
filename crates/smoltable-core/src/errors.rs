use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmoltableError {
    #[error("Invalid retrieval key")]
    InvalidRetrievalKey,
}

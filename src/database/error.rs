
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Could not create user")]
    CreationError,
    #[error("Could not delete User")]
    DeletionError
}

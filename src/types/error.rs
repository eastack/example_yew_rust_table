//! Error type for error handling

use thiserror::Error as ThisError;

/// Define all possible errors
#[derive(ThisError, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// request error
    #[error("Http Request Error")]
    RequestError(String),
}

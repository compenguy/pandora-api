/*!
*/
// SPDX-License-Identifier: MIT
use crate::json::errors::JsonError;

use thiserror::Error;

/// A general, crate-wide result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type capturing the full range of errors that can arise in the use of
/// this API.  The PandoraJsonRequestError variant wraps a separate error type
/// (json::errors::JsonError) that captures errors returned by the Pandora
/// JSON API.
#[derive(Error, Debug)]
pub enum Error {
    /// Wraps serde_json serialization/deserializaiton errors
    #[error("JSON serialization error: {0}")]
    JsonSerializationError(#[from] serde_json::error::Error),
    /// Wraps reqwest errors
    #[error("HTTP I/O error: {0}")]
    HttpIoError(surf::Error),
    /// Wraps another error type that describes API errors returned by the
    /// Pandora JSON API
    #[error("Pandora API error: {0}")]
    PandoraJsonRequestError(#[from] JsonError),
    /// Invalid/unsupported audio format was specified
    #[error("Invalid/unsupported audio format: {0}")]
    InvalidAudioFormat(String),
    /// Invalid/unsupported gender string was specified
    #[error("Invalid/unsupported gender value: {0}")]
    InvalidUserGender(String),
}

impl From<surf::Error> for Error {
    fn from(err: surf::Error) -> Self {
        Error::HttpIoError(err)
    }
}

/*!
*/
// SPDX-License-Identifier: MIT
use crate::json::errors::JsonError;

/// A general, crate-wide result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type capturing the full range of errors that can arise in the use of
/// this API.  The PandoraJsonRequestError variant wraps a separate error type
/// (json::errors::JsonError) that captures errors returned by the Pandora
/// JSON API.
#[derive(Debug)]
pub enum Error {
    /// Wraps serde_json serialization/deserializaiton errors
    JsonSerializationError(serde_json::error::Error),
    /// Wraps reqwest errors
    HttpIoError(reqwest::Error),
    /// Wraps another error type that describes API errors returned by the
    /// Pandora JSON API
    PandoraJsonRequestError(JsonError),
    /// Invalid/unsupported audio format was specified
    InvalidAudioFormat(String),
}

impl std::error::Error for Error {
    /// Return the source for this error, if any
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::JsonSerializationError(e) => Some(e),
            Error::HttpIoError(e) => Some(e),
            Error::PandoraJsonRequestError(e) => Some(e),
            Error::InvalidAudioFormat(_) => None,
        }
    }
}

impl std::fmt::Display for Error {
    /// Format this error for display
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::JsonSerializationError(e) => write!(f, "JSON serialization error: {}", e),
            Error::HttpIoError(e) => write!(f, "HTTP I/O error: {}", e),
            Error::PandoraJsonRequestError(e) => write!(f, "Pandora API error: {}", e),
            Error::InvalidAudioFormat(fmt) => {
                write!(f, "Invalid/unsupported audio format: {}", fmt)
            }
        }
    }
}

impl From<JsonError> for Error {
    /// Create the appropriate instance of this error type from JsonErrors.
    fn from(err: JsonError) -> Self {
        Error::PandoraJsonRequestError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    /// Create the appropriate instance of this error type from serde_json errors.
    fn from(err: serde_json::error::Error) -> Self {
        Error::JsonSerializationError(err)
    }
}

impl From<reqwest::Error> for Error {
    /// Create the appropriate instance of this error type from reqwest errors.
    fn from(err: reqwest::Error) -> Self {
        Error::HttpIoError(err)
    }
}

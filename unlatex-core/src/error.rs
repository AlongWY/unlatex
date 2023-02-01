#![allow(missing_docs)]
//! Error handling for this crate.

use std::ffi::NulError;
use std::ops::Range;
use std::str::Utf8Error;

/// Error type for this crate.
#[non_exhaustive]
#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    /// Could not allocate memory
    /// This is generally only triggered when out of memory.
    #[error("failed to allocate memory")]
    Allocation,
    /// Found a string with a internal null byte while converting
    /// to C string.
    #[error("found a string with a internal null byte while converting to C string")]
    InvalidString(NulError),
    /// String from rquickjs was not UTF-8
    #[error("string from rquickjs was not UTF-8")]
    Utf8(Utf8Error),
    /// An io error
    #[error("an io error")]
    Io(String),
    /// An exception raised by quickjs itself.
    #[error("an exception raised by quickjs itself (details: {message} at {file}:{line})")]
    Exception {
        message: String,
        file: String,
        line: i32,
        stack: String,
    },
    /// Error converting from javascript to a rust type.
    #[error("error converting from javascript to a rust type")]
    FromJs {
        from: &'static str,
        to: &'static str,
        message: Option<String>,
    },
    /// Error converting to javascript from a rust type.
    #[error("error converting to javascript from a rust type")]
    IntoJs {
        from: &'static str,
        to: &'static str,
        message: Option<String>,
    },
    /// Error matching of function arguments
    #[error("error matching of function arguments (details: expected {expected:?} arguments, but got {given})")]
    NumArgs {
        expected: Range<usize>,
        given: usize,
    },
    #[cfg(feature = "loader")]
    /// Error when resolving js module
    #[error("error when resolving js module (details: {message} from {base} to {name})")]
    Resolving {
        base: String,
        name: String,
        message: Option<String>,
    },
    #[cfg(feature = "loader")]
    /// Error when loading js module
    #[error("error when loading js module (details: {message} from {name})")]
    Loading {
        name: String,
        message: Option<String>,
    },
    /// Error when restoring a Persistent in a runtime other than the original runtime.
    #[error("error when restoring a Persistent in a runtime other than the original runtime")]
    UnrelatedRuntime,
    /// An error from quickjs from which the specifics are unknown.
    /// Should eventually be removed as development progresses.
    #[error("an error from quickjs from which the specifics are unknown")]
    Unknown,
}

/// Alias to `core::result::Result<T, unlatex-core::Error>`
pub type Result<T, E = Error> = core::result::Result<T, E>;


impl From<rquickjs::Error> for Error {
    fn from(e: rquickjs::Error) -> Self {
        match e {
            rquickjs::Error::Allocation => { Self::Allocation }
            rquickjs::Error::InvalidString(e) => {
                Self::InvalidString(e)
            }
            rquickjs::Error::Utf8(e) => {
                Self::Utf8(e)
            }
            rquickjs::Error::Io(e) => {
                Self::Io(e.to_string())
            }
            rquickjs::Error::Exception { message, file, line, stack } => {
                Self::Exception { message, file, line, stack }
            }
            rquickjs::Error::FromJs { from, to, message } => {
                Self::FromJs { from, to, message }
            }
            rquickjs::Error::IntoJs { from, to, message } => {
                Self::IntoJs { from, to, message }
            }
            rquickjs::Error::NumArgs { expected, given } => {
                Self::NumArgs { expected, given }
            }
            #[cfg(feature = "loader")]
            rquickjs::Error::Resolving { base, name, message } => {
                Self::Resolving { base, name, message }
            }
            #[cfg(feature = "loader")]
            rquickjs::Error::Loading { name, message } => {
                Self::Loading { name, message }
            }
            rquickjs::Error::UnrelatedRuntime => {
                Self::UnrelatedRuntime
            }
            _ => {
                Self::Unknown
            }
        }
    }
}


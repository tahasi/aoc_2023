use std::{ffi::OsStr, io};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PuzzleError {
    #[error("load input file '{path}' failure: {source}")]
    LoadInputFailure {
        path: String,
        source: std::io::Error,
    },
    #[error("invalid input at {line}: {reason}")]
    InvalidInput { line: usize, reason: String },
    #[error("not implemented")]
    NotImplemented,
    #[error("{message}")]
    Unexpected { message: String },
    #[error("{message}: {source}")]
    UnexpectedErr {
        message: String,
        #[source]
        source: anyhow::Error,
    },
}

impl PuzzleError {
    pub(crate) fn from_io_error<S: AsRef<OsStr> + ?Sized>(path: &S, source: io::Error) -> Self {
        let path = path.as_ref().to_string_lossy().to_string();
        PuzzleError::LoadInputFailure { path, source }
    }

    pub(crate) fn invalid_input(line: usize, reason: &str) -> Self {
        let reason = reason.to_owned();
        PuzzleError::InvalidInput { line, reason }
    }

    pub(crate) fn unexpected(message: &str) -> Self {
        let message = message.to_owned();
        PuzzleError::Unexpected { message }
    }

    #[allow(dead_code)]
    pub(crate) fn unexpected_err(message: &str, err: anyhow::Error) -> Self {
        let message = message.to_owned();
        PuzzleError::UnexpectedErr {
            message,
            source: err,
        }
    }
}

impl PartialEq for PuzzleError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::LoadInputFailure {
                    path: l_path,
                    source: l_source,
                },
                Self::LoadInputFailure {
                    path: r_path,
                    source: r_source,
                },
            ) => l_path == r_path && l_source.to_string() == r_source.to_string(),
            (
                Self::InvalidInput {
                    line: l_line,
                    reason: l_reason,
                },
                Self::InvalidInput {
                    line: r_line,
                    reason: r_reason,
                },
            ) => l_line == r_line && l_reason == r_reason,
            (Self::Unexpected { message: l_message }, Self::Unexpected { message: r_message }) => {
                l_message == r_message
            }
            (
                Self::UnexpectedErr {
                    message: l_message,
                    source: l_source,
                },
                Self::UnexpectedErr {
                    message: r_message,
                    source: r_source,
                },
            ) => l_message == r_message && l_source.to_string() == r_source.to_string(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

pub type Result<T> = core::result::Result<T, PuzzleError>;

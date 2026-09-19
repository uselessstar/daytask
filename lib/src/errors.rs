use thiserror::Error;

/// Errors that can occur while creating or updating a [Task](crate::Task).
#[derive(Debug, Error, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TaskError {
    /// The task name is empty or contains only whitespace.
    #[error("task name cannot be empty")]
    EmptyName,
    /// The task contains an invalid status value.
    #[error("invalid status value: {0}")]
    InvalidStatusValue(u8),
    /// The task contains an invalid status string.
    #[error("invalid status string: {0}")]
    InvalidStatusString(String),
}

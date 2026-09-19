use thiserror::Error;

/// Errors that can occur while creating or updating a [Task](crate::Task).
#[derive(Debug, Error, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[expect(
    missing_copy_implementations,
    reason = "task error is an enum representing errors related to task operations and does not need to be copyable"
)]
pub enum TaskError {
    /// The task name is empty or contains only whitespace.
    #[error("task name cannot be empty")]
    EmptyName,
}

/// Errors that can occur while creating a [Status](crate::Status).
#[derive(Debug, Error, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum StatusError {
    /// The status contains a invalid value.
    #[error("invalid status value: {0}")]
    InvalidStatusValue(u8),
    /// The status contains a invalid string.
    #[error("invalid status string: {0}")]
    InvalidStatusString(String),
}

use thiserror::Error;

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum TaskIdError {
    #[error("TaskId cannot be zero")]
    Zero,
}

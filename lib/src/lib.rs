#![doc = include_str!("../README.md")]
#![cfg_attr(
    not(debug_assertions),
    deny(
        clippy::allow_attributes_without_reason,
        missing_docs,
        unused,
        clippy::perf,
        clippy::suspicious
    )
)]

use uuid::Uuid;

/// todo
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Task {
    id: Uuid,
}

impl Task {
    /// todo
    pub fn new(id: impl Into<Uuid>) -> Task {
        Self { id: id.into() }
    }

    /// todo
    pub fn id(&self) -> Uuid {
        self.id
    }
}

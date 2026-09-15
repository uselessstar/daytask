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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Task {
    id: Uuid,
    name: String,
}

impl Task {
    /// todo
    pub fn new(id: impl Into<Uuid>, name: impl Into<String>) -> Task {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }

    /// todo
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// todo
    pub fn name(&self) -> &str {
        &self.name
    }

    /// todo
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }
}

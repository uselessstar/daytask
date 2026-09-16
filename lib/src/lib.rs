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
#[derive(Debug, Default)]
pub struct TaskBuilder {
    name: String,
    id: Option<Uuid>,
}

impl TaskBuilder {
    /// todo
    pub fn new() -> Self {
        Self::default()
    }

    /// todo
    pub fn with_id(mut self, id: impl Into<Uuid>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// todo
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// todo
    pub fn build(self) -> Task {
        Task {
            id: self.id.unwrap_or_else(Uuid::now_v7),
            name: self.name,
        }
    }
}

/// todo
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Task {
    id: Uuid,
    name: String,
}

impl Task {
    /// todo
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::now_v7(),
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

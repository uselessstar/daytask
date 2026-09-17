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
    pub fn new(name: impl Into<String>) -> Self {
        let task = Self {
            id: Uuid::now_v7(),
            name: name.into(),
        };
        debug_log!(target: "task","Task created: {:#?}", task);
        task
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
        let old = std::mem::replace(&mut self.name, name.into());
        debug_log!(target: "task", "Task name changed: id={}, old={:?}, new={:?}", self.id, old, self.name);
    }
}

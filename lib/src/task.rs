use uuid::Uuid;

/// todo
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Task {
    id: Uuid,
    name: String,
}

impl Task {
    /// Creates a new [Task].
    ///
    /// # Arguments
    /// - **name**: The name to set to the [task](Task).
    ///
    /// # Example
    /// ```
    /// # use daytask::Task;
    /// #
    /// let task = Task::new("example");
    /// let clone_task = task.clone();
    ///
    /// assert_eq!(clone_task,task);
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        let task = Self {
            id: Uuid::now_v7(),
            name: name.into(),
        };
        debug_log!(target: "task","Task created: {:#?}", task);
        task
    }

    /// Returns the id of the [task](Task).
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Returns the name of the [task](Task).
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the [task](Task).
    ///
    /// # Arguments
    /// - **name**: The new name to set.
    ///
    /// # Example
    /// ```
    /// # use daytask::Task;
    /// #
    /// let mut task = Task::new("example");
    /// task.set_name("example 2");
    ///
    /// assert_eq!(task.name(), "example 2");
    /// ```
    pub fn set_name(&mut self, name: impl Into<String>) {
        let old = std::mem::replace(&mut self.name, name.into());
        debug_log!(target: "task", "Task name changed: id={}, old={:?}, new={:?}", self.id, old, self.name);
    }
}

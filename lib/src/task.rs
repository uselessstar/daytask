use uuid::Uuid;

/// todo
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Task {
    id: Uuid,
    name: String,
    description: Option<String>,
}

impl Task {
    /// Creates a new [Task] with the specified name.
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
            description: None,
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

    /// Returns the description of the [task](Task).
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
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
        #[cfg(feature = "log")]
        {
            let old = std::mem::replace(&mut self.name, name.into());
            debug_log!(target: "task", "Task name changed: id={}, old={:?}, new={:?}", self.id, old, self.name);
        }
        #[cfg(not(feature = "log"))]
        {
            self.name = name.into();
        }
    }

    /// Sets the description of the [task](Task).
    ///
    /// # Arguments
    /// - **description**: The new description to set.
    ///
    /// # Example
    /// ```
    /// # use daytask::Task;
    /// #
    /// let mut task = Task::new("example");
    /// task.set_description("This is a description");
    ///
    /// assert_eq!(task.description(), Some("This is a description"));
    /// ```
    pub fn set_description(&mut self, description: impl Into<String>) {
        #[cfg(feature = "log")]
        {
            let old = self.description.replace(description.into());
            debug_log!(target: "task", "Task description changed: id={}, old={:?}, new={:?}", self.id, old, self.description);
        }
        #[cfg(not(feature = "log"))]
        {
            self.description = Some(description.into());
        }
    }

    /// Clears the description of the [task](Task).
    ///
    /// # Example
    /// ```
    /// # use daytask::Task;
    /// #
    /// let mut task = Task::new("example");
    /// task.set_description("This is a description");
    /// task.clear_description();
    ///
    /// assert_eq!(task.description(), None);
    /// ```
    pub fn clear_description(&mut self) {
        #[cfg(feature = "log")]
        {
            let old = self.description.take();
            debug_log!(target: "task", "Task description cleared: id={}, old={:?}", self.id, old);
        }
        #[cfg(not(feature = "log"))]
        {
            self.description = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test Task");

        assert_eq!(task.name(), "Test Task");
        assert_ne!(task.id(), Uuid::nil());
    }

    #[test]
    fn test_unique_ids() {
        let task1 = Task::new("Task #1");
        let task2 = Task::new("Task #2");

        assert_ne!(task1.id(), task2.id());
    }

    #[test]
    fn test_set_name() {
        let mut task = Task::new("Initial Name");
        task.set_name("Updated Name");

        assert_eq!(task.name(), "Updated Name");
    }

    #[test]
    fn test_description_starts_empty_and_can_be_set() {
        let mut task = Task::new("Description Test");

        assert_eq!(task.description(), None);

        task.set_description("Task details");

        assert_eq!(task.description(), Some("Task details"));
    }

    #[test]
    fn test_description_can_be_replaced_and_removed() {
        let mut task = Task::new("Description Update Test");

        task.set_description("Initial details");
        task.set_description(String::from("Updated details"));

        assert_eq!(task.description(), Some("Updated details"));

        task.clear_description();
        assert_eq!(task.description(), None);

        task.clear_description();
        assert_eq!(task.description(), None);
    }

    #[test]
    fn test_mutations_preserve_task_identity() {
        let mut task = Task::new("Identity Test");
        let id = task.id();

        task.set_name("Renamed");
        task.set_description("Details");
        task.clear_description();

        assert_eq!(task.id(), id);
    }

    #[test]
    fn test_equality_and_cloning() {
        let task = Task::new("Clone Test");
        let cloned_task = task.clone();

        assert_eq!(task, cloned_task);
        assert_eq!(task.id(), cloned_task.id());
        assert_eq!(task.name(), cloned_task.name());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_serialization() {
        let mut task = Task::new("Serde Test");
        task.set_description("Serialized details");
        let serialized = serde_json::to_string(&task).expect("Serialization Failed");
        let deserialized: Task = serde_json::from_str(&serialized).expect("Deserialization Failed");

        assert_eq!(task, deserialized);
    }
}

use std::fmt::Display;

use crate::TaskError;
use uuid::Uuid;

/// A specialized [Result] type for task operations.
type Result<T> = std::result::Result<T, TaskError>;

/// Indicates the current status of a [task](Task).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
#[non_exhaustive]
pub enum Status {
    /// Pending status indicates that the task has not yet been started.
    #[default]
    Pending = 0,
    /// In Progress status indicates that the task is currently being worked on.
    InProgress = 1,
    /// Completed status indicates that the task has been finished.
    Completed = 2,
}

impl Status {
    /// Returns `true` if the status is [Pending](Status::Pending), `false` otherwise.
    #[must_use]
    #[inline]
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns `true` if the status is [InProgress](Status::InProgress), `false` otherwise.
    #[inline]
    #[must_use]
    pub fn is_in_progress(&self) -> bool {
        matches!(self, Self::InProgress)
    }

    /// Returns `true` if the status is [Completed](Status::Completed), `false` otherwise.
    #[inline]
    #[must_use]
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed)
    }
}

impl TryFrom<u8> for Status {
    type Error = TaskError;

    #[inline]
    fn try_from(value: u8) -> Result<Self> {
        match value {
            x if x == Self::Pending as u8 => Ok(Self::Pending),
            x if x == Self::InProgress as u8 => Ok(Self::InProgress),
            x if x == Self::Completed as u8 => Ok(Self::Completed),
            _ => Err(TaskError::InvalidStatusValue(value)),
        }
    }
}

impl From<Status> for u8 {
    #[inline]
    fn from(status: Status) -> Self {
        status as Self
    }
}

impl Display for Status {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            Self::Pending => "Pending",
            Self::InProgress => "In Progress",
            Self::Completed => "Completed",
        };
        write!(f, "{}", status_str)
    }
}

/// Represents a task with a unique identifier, name, optional description, and status.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Task {
    id: Uuid,
    name: String,
    description: Option<String>,
    status: Status,
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Task {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct TaskHelper {
            id: Uuid,
            name: String,
            description: Option<String>,
            status: Status,
        }

        let helper = TaskHelper::deserialize(deserializer)?;

        if helper.id.is_nil() {
            return Err(serde::de::Error::custom("task id cannot be nil"));
        }
        if helper.name.trim().is_empty() {
            return Err(serde::de::Error::custom("task name cannot be empty"));
        }
        Ok(Self {
            id: helper.id,
            name: helper.name,
            description: helper.description,
            status: helper.status,
        })
    }
}

impl Task {
    /// Creates a new [Task] with the specified name.
    ///
    /// # Arguments
    /// - **name**: The name to set to the [task](Task).
    ///
    /// # Errors
    /// Returns [`TaskError::EmptyName`] when the name is empty or contains only whitespace.
    ///
    /// # Example
    /// ```
    /// # use daytask::Task;
    /// #
    /// let task = Task::new("example").expect("valid task name");
    /// let clone_task = task.clone();
    ///
    /// assert_eq!(clone_task,task);
    /// ```
    #[inline]
    pub fn new(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(TaskError::EmptyName);
        }

        let id: Uuid;

        #[cfg(feature = "v7")]
        {
            id = Uuid::now_v7();
        }
        #[cfg(feature = "v4")]
        {
            id = Uuid::new_v4();
        }

        let task = Self {
            id,
            name,
            description: None,
            status: Status::default(),
        };
        debug_log!(target: "task", "task created: {:#?}", task);
        Ok(task)
    }

    /// Returns the id of the [task](Task).
    #[must_use]
    #[inline]
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Returns the name of the [task](Task).
    #[must_use]
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the status of the [task](Task).
    #[must_use]
    #[inline]
    pub fn status(&self) -> Status {
        self.status
    }

    /// Returns the description of the [task](Task).
    #[must_use]
    #[inline]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Sets the name of the [task](Task).
    ///
    /// # Arguments
    /// - **name**: The new name to set.
    ///
    /// # Errors
    /// Returns [`TaskError::EmptyName`] when the name is empty or contains only whitespace.
    ///
    /// # Example
    /// ```
    /// # use daytask::Task;
    /// #
    /// let mut task = Task::new("example").expect("valid task name");
    /// task.set_name("example 2").expect("valid task name");
    ///
    /// assert_eq!(task.name(), "example 2");
    /// ```
    #[inline]
    pub fn set_name(&mut self, name: impl AsRef<str>) -> Result<()> {
        let name = name.as_ref();
        if name.trim().is_empty() {
            return Err(TaskError::EmptyName);
        }
        if self.name == name {
            return Ok(());
        }
        #[cfg(feature = "log")]
        {
            let old = std::mem::replace(&mut self.name, name.into());
            debug_log!(target: "task", "task name changed: id={}, old={:?}, new={:?}", self.id, old, self.name);
        }
        #[cfg(not(feature = "log"))]
        {
            self.name = name.into();
        }
        Ok(())
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
    /// let mut task = Task::new("example").expect("valid task name");
    /// task.set_description("This is a description");
    ///
    /// assert_eq!(task.description(), Some("This is a description"));
    /// ```
    #[inline]
    pub fn set_description(&mut self, description: impl AsRef<str>) {
        let description = description.as_ref();
        if self.description.as_deref() == Some(description) {
            return;
        }

        #[cfg(feature = "log")]
        {
            let old = self.description.replace(description.to_owned());
            debug_log!(target: "task", "task description changed: id={}, old={:?}, new={:?}", self.id, old, self.description);
        }
        #[cfg(not(feature = "log"))]
        {
            self.description = Some(description.to_owned());
        }
    }

    /// Clears the description of the [task](Task).
    ///
    /// # Example
    /// ```
    /// # use daytask::Task;
    /// #
    /// let mut task = Task::new("example").expect("valid task name");
    /// task.set_description("This is a description");
    /// task.clear_description();
    ///
    /// assert_eq!(task.description(), None);
    /// ```
    #[inline]
    pub fn clear_description(&mut self) {
        #[cfg(feature = "log")]
        {
            let old = self.description.take();
            debug_log!(target: "task", "task description cleared: id={}, old={:?}", self.id, old);
        }
        #[cfg(not(feature = "log"))]
        {
            self.description = None;
        }
    }

    /// Sets the status of the [task](Task).
    ///
    /// # Arguments
    /// - **status**: The new status to set.
    ///
    /// # Example
    /// ```
    /// # use daytask::{Task, Status};
    /// #
    /// let mut task = Task::new("example").expect("valid task name");
    /// task.set_status(Status::InProgress);
    ///
    /// assert_eq!(task.status(), Status::InProgress);
    /// ```
    #[inline]
    pub fn set_status(&mut self, status: Status) {
        if self.status == status {
            return;
        }
        #[cfg(feature = "log")]
        {
            let old = std::mem::replace(&mut self.status, status);
            debug_log!(target: "task", "task status changed: id={}, old={:?}, new={:?}", self.id, old, self.status);
        }
        #[cfg(not(feature = "log"))]
        {
            self.status = status;
        }
    }

    /// Returns true if the [task](Task) is pending.
    #[inline]
    #[must_use]
    pub fn is_pending(&self) -> bool {
        self.status.is_pending()
    }

    /// Returns true if the [task](Task) is in progress.
    #[must_use]
    #[inline]
    pub fn is_in_progress(&self) -> bool {
        self.status.is_in_progress()
    }

    /// Returns true if the [task](Task) is completed.
    #[must_use]
    #[inline]
    pub fn is_completed(&self) -> bool {
        self.status.is_completed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test Task").expect("valid task name");

        assert_eq!(task.name(), "Test Task");
        assert_ne!(task.id(), Uuid::nil());
        assert_eq!(task.status(), Status::Pending);
        assert!(task.is_pending());
        assert!(!task.is_in_progress());
        assert!(!task.is_completed());
    }

    #[test]
    fn test_status_predicates() {
        assert!(Status::Pending.is_pending());
        assert!(!Status::Pending.is_in_progress());
        assert!(!Status::Pending.is_completed());

        assert!(!Status::InProgress.is_pending());
        assert!(Status::InProgress.is_in_progress());
        assert!(!Status::InProgress.is_completed());

        assert!(!Status::Completed.is_pending());
        assert!(!Status::Completed.is_in_progress());
        assert!(Status::Completed.is_completed());
    }

    #[test]
    fn test_status_can_be_changed() {
        let mut task = Task::new("Status Test").expect("valid task name");

        task.set_status(Status::InProgress);
        assert_eq!(task.status(), Status::InProgress);
        assert!(task.is_in_progress());

        task.set_status(Status::Completed);
        assert_eq!(task.status(), Status::Completed);
        assert!(task.is_completed());
    }

    #[test]
    fn test_setting_same_status_is_a_no_op() {
        let mut task = Task::new("Same Status Test").expect("valid task name");

        task.set_status(Status::Pending);

        assert_eq!(task.status(), Status::Pending);
    }

    #[test]
    fn test_id_version_matches_feature() {
        let task = Task::new("UUID Version Test").expect("valid task name");

        #[cfg(feature = "v4")]
        assert_eq!(task.id().get_version(), Some(uuid::Version::Random));

        #[cfg(feature = "v7")]
        assert_eq!(task.id().get_version(), Some(uuid::Version::SortRand));
    }

    #[test]
    fn test_unique_ids() {
        let task1 = Task::new("Task #1").expect("valid task name");
        let task2 = Task::new("Task #2").expect("valid task name");

        assert_ne!(task1.id(), task2.id());
    }

    #[test]
    fn test_set_name() {
        let mut task = Task::new("Initial Name").expect("valid task name");
        task.set_name("Updated Name").expect("valid task name");

        assert_eq!(task.name(), "Updated Name");
    }

    #[test]
    fn test_setting_same_name_is_a_no_op() {
        let mut task = Task::new("Same Name").expect("valid task name");
        let id = task.id();

        task.set_name("Same Name").expect("valid task name");

        assert_eq!(task.name(), "Same Name");
        assert_eq!(task.id(), id);
    }

    #[test]
    fn test_empty_name_is_rejected() {
        assert_eq!(Task::new("   "), Err(TaskError::EmptyName));

        let mut task = Task::new("Valid Name").expect("valid task name");
        assert_eq!(task.set_name("\t"), Err(TaskError::EmptyName));
        assert_eq!(task.name(), "Valid Name");
    }

    #[test]
    fn test_description_starts_empty_and_can_be_set() {
        let mut task = Task::new("Description Test").expect("valid task name");

        assert_eq!(task.description(), None);

        task.set_description("Task details");

        assert_eq!(task.description(), Some("Task details"));
    }

    #[test]
    fn test_setting_same_description_is_a_no_op() {
        let mut task = Task::new("Description Noop Test").expect("valid task name");

        task.set_description("Same description");
        let id = task.id();

        task.set_description("Same description");

        assert_eq!(task.id(), id);
        assert_eq!(task.description(), Some("Same description"));
    }

    #[test]
    fn test_description_can_be_replaced_and_removed() {
        let mut task = Task::new("Description Update Test").expect("valid task name");

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
        let mut task = Task::new("Identity Test").expect("valid task name");
        let id = task.id();

        task.set_name("Renamed").expect("valid task name");
        task.set_description("Details");
        task.clear_description();

        assert_eq!(task.id(), id);
    }

    #[test]
    fn test_equality_and_cloning() {
        let task = Task::new("Clone Test").expect("valid task name");
        let cloned_task = task.clone();

        assert_eq!(task, cloned_task);
        assert_eq!(task.id(), cloned_task.id());
        assert_eq!(task.name(), cloned_task.name());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_serialization() {
        let mut task = Task::new("Serde Test").expect("valid task name");
        task.set_description("Serialized details");
        let serialized = serde_json::to_string(&task).expect("serialization failed");
        let deserialized: Task = serde_json::from_str(&serialized).expect("deserialization failed");

        assert_eq!(task, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_rejects_empty_names() {
        for name in ["", "   "] {
            let serialized = serde_json::json!({
                "id": "00000000-0000-0000-0000-000000000001",
                "name": name,
                "description": null,
                "status": "Pending"
            });

            let result = serde_json::from_value::<Task>(serialized);

            assert!(result.is_err(), "expected invalid name to be rejected: {name:?}");
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_rejects_nil_ids() {
        let serialized = serde_json::json!({
            "id": "00000000-0000-0000-0000-000000000000",
            "name": "Valid name",
            "description": null,
            "status": "Pending"
        });

        let result = serde_json::from_value::<Task>(serialized);

        assert!(result.is_err(), "expected nil task id to be rejected");
    }

    #[test]
    fn test_status_try_from_valid_values() {
        assert_eq!(Status::try_from(0u8), Ok(Status::Pending));
        assert_eq!(Status::try_from(1u8), Ok(Status::InProgress));
        assert_eq!(Status::try_from(2u8), Ok(Status::Completed));
    }

    #[test]
    fn test_status_try_from_invalid_values() {
        assert_eq!(Status::try_from(3u8), Err(TaskError::InvalidStatusValue(3)));
        assert_eq!(Status::try_from(255u8), Err(TaskError::InvalidStatusValue(255)));
    }

    #[test]
    fn test_status_roundtrip() {
        for status in [Status::Pending, Status::InProgress, Status::Completed] {
            let value = status as u8;
            assert_eq!(Status::try_from(value), Ok(status));
        }
    }
}

use core::fmt::{self, Display, Formatter};

use crate::TaskError;
use uuid::Uuid;

mod status;
#[cfg(test)]
mod tests;

pub use status::Status;

/// A specialized [Result] type for task operations.
type Result<T> = core::result::Result<T, TaskError>;

/// Represents a task with a unique identifier, name, optional description, and status.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Task {
    id: Uuid,
    name: String,
    description: Option<String>,
    status: Status,
}

impl Display for Task {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.name, self.status)?;
        if let Some(desc) = self.description.as_deref() {
            write!(f, ": {desc}")?;
        }
        Ok(())
    }
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
            let old = core::mem::replace(&mut self.name, name.into());
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
        if self.description.is_none() {
            return;
        }
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
            let old = core::mem::replace(&mut self.status, status);
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

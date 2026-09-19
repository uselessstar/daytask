use crate::StatusError;
use core::fmt::Display;
use core::str::FromStr;

type Result<T> = core::result::Result<T, StatusError>;

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
    type Error = StatusError;

    #[inline]
    fn try_from(value: u8) -> Result<Self> {
        match value {
            x if x == Self::Pending as u8 => Ok(Self::Pending),
            x if x == Self::InProgress as u8 => Ok(Self::InProgress),
            x if x == Self::Completed as u8 => Ok(Self::Completed),
            _ => Err(StatusError::InvalidStatusValue(value)),
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let status_str = match self {
            Self::Pending => "Pending",
            Self::InProgress => "In Progress",
            Self::Completed => "Completed",
        };
        write!(f, "{}", status_str)
    }
}

impl AsRef<str> for Status {
    #[inline]
    fn as_ref(&self) -> &str {
        match self {
            Self::Pending => "Pending",
            Self::InProgress => "In Progress",
            Self::Completed => "Completed",
        }
    }
}

impl FromStr for Status {
    type Err = StatusError;

    #[inline]
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Pending" => Ok(Self::Pending),
            "In Progress" => Ok(Self::InProgress),
            "Completed" => Ok(Self::Completed),
            _ => Err(StatusError::InvalidStatusString(s.to_owned())),
        }
    }
}

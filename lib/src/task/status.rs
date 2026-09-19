use crate::StatusError;
use core::fmt::Display;
use core::str::FromStr;

type Result<T> = core::result::Result<T, StatusError>;

/// Indicates the current status of a [task](crate::Task).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

#[cfg(feature = "serde")]
impl serde::Serialize for Status {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Status {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StatusVisitor;

        impl serde::de::Visitor<'_> for StatusVisitor {
            type Value = Status;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("a valid task status string")
            }

            fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                value.parse().map_err(E::custom)
            }
        }

        deserializer.deserialize_str(StatusVisitor)
    }
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

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use core::str::FromStr;

    use crate::Status;
    use crate::StatusError;

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
    fn test_status_try_from_valid_values() {
        assert_eq!(Status::try_from(0u8), Ok(Status::Pending));
        assert_eq!(Status::try_from(1u8), Ok(Status::InProgress));
        assert_eq!(Status::try_from(2u8), Ok(Status::Completed));
    }

    #[test]
    fn test_status_try_from_invalid_values() {
        assert_eq!(Status::try_from(3u8), Err(StatusError::InvalidStatusValue(3)));
        assert_eq!(Status::try_from(255u8), Err(StatusError::InvalidStatusValue(255)));
    }

    #[test]
    fn test_status_roundtrip() {
        for status in [Status::Pending, Status::InProgress, Status::Completed] {
            let value = status as u8;
            assert_eq!(Status::try_from(value), Ok(status));
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_status_deserialize() {
        let status_strs = ["Pending", "In Progress", "Completed"];
        let expected_statuses = [Status::Pending, Status::InProgress, Status::Completed];

        for (status_str, expected_status) in status_strs.iter().zip(expected_statuses.iter()) {
            assert_eq!(Status::from_str(status_str).unwrap(), *expected_status);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_status_serialize() {
        let sample = [Status::Completed, Status::InProgress, Status::Pending];
        let expected_json = ["\"Completed\"", "\"In Progress\"", "\"Pending\""];

        for (status, expected) in sample.iter().zip(expected_json.iter()) {
            let status_json = serde_json::to_string(status).unwrap();
            assert_eq!(status_json, *expected);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_status_deserialize_rejects_invalid_string() {
        let result = serde_json::from_str::<Status>("\"Unknown\"");

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid status string: Unknown")
        );
    }
}

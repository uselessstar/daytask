use core::fmt::{Display, Formatter, Result as FmtResult};
use core::num::NonZeroU64;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering::Relaxed;

mod error;

pub use error::TaskIdError;

type Result<T> = core::result::Result<T, TaskIdError>;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(NonZeroU64);

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

impl TaskId {
    #[expect(
        clippy::new_without_default,
        reason = "TaskId is a unique identifier generator, not a default-constructible value"
    )]
    pub fn new() -> Self {
        let id: u64 = NEXT_ID.fetch_add(1, Relaxed);
        Self(NonZeroU64::new(id).expect("TaskId counter must never start at or reach zero"))
    }
}

impl From<TaskId> for u64 {
    fn from(value: TaskId) -> Self {
        value.0.into()
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl TryFrom<u64> for TaskId {
    type Error = TaskIdError;

    fn try_from(value: u64) -> Result<Self> {
        NonZeroU64::new(value).map(Self).ok_or(TaskIdError::Zero)
    }
}

#[cfg(test)]
mod tests {
    use super::TaskId;
    #[test]
    fn new_is_unique() {
        let id = TaskId::new();
        let id2 = TaskId::new();
        assert_ne!(id, id2);
    }

    #[test]
    fn from_taskid_to_u64() {
        let id = TaskId::new();
        let u64_id = u64::from(id);
        assert_eq!(u64::from(id), u64_id);
    }

    #[test]
    fn display_taskid() {
        let id = TaskId::new();
        let id_str = format!("{}", id);
        assert_eq!(id_str, id.0.to_string());
    }
}

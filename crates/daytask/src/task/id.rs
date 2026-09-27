use core::fmt::{Display, Formatter, Result as FmtResult};
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering::Relaxed;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

impl TaskId {
    #[expect(
        clippy::new_without_default,
        reason = "TaskId is a unique identifier generator, not a default-constructible value"
    )]
    pub fn new() -> Self {
        let id: u64 = NEXT_ID.fetch_add(1, Relaxed);
        Self(id)
    }
}

impl From<TaskId> for u64 {
    fn from(value: TaskId) -> Self {
        value.0
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

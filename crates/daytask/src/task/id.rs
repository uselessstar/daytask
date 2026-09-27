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
        assert_eq!(id.0, u64_id);
    }

    #[test]
    fn display_taskid() {
        let id = TaskId::new();
        let id_str = format!("{}", id);
        assert_eq!(id_str, id.0.to_string());
    }
}

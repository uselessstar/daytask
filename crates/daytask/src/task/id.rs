use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering::Relaxed;

#[repr(transparent)]
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

    #[must_use]
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

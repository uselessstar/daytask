use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering::Relaxed;

#[repr(transparent)]
pub struct TaskId(u64);

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

impl TaskId {
    pub fn new() -> Self {
        let id: u64 = NEXT_ID.fetch_add(1, Relaxed);
        Self(id)
    }
}

use core::sync::atomic::AtomicU64;

#[repr(transparent)]
pub struct TaskId(u64);

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

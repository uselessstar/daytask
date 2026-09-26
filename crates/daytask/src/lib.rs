#![doc = include_str!("../README.md")]

pub struct Task {
    id: TaskId,
}

pub struct TaskId(u64);

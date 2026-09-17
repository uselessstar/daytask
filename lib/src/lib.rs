#![doc = include_str!("../README.md")]
#![cfg_attr(
    not(debug_assertions),
    deny(
        clippy::allow_attributes_without_reason,
        missing_docs,
        unused,
        clippy::perf,
        clippy::suspicious,
        clippy::missing_errors_doc
    )
)]

#[macro_use]
mod macros;
mod task;

pub use task::{Task, TaskError};

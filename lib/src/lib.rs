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

#[cfg(all(feature = "v4", feature = "v7"))]
compile_error!("features `v4` and `v7` cannot be enabled at the same time");

#[cfg(not(any(feature = "v4", feature = "v7")))]
compile_error!("either feature `v4` or feature `v7` must be enabled");

#[macro_use]
mod macros;
mod task;

pub use task::{Task, TaskError};

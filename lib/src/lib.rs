#![doc = include_str!("../README.md")]

#[cfg(all(feature = "v4", feature = "v7"))]
compile_error!("features `v4` and `v7` cannot be enabled at the same time");

#[cfg(not(any(feature = "v4", feature = "v7")))]
compile_error!("either feature `v4` or feature `v7` must be enabled");

#[macro_use]
mod macros;
mod errors;
mod task;

pub use errors::{StatusError, TaskError};
pub use task::{Status, Task};

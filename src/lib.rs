//! Wrappers over OS sync primitives

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

mod sem;
pub use sem::{Sem, Semaphore, SemaphoreGuard};

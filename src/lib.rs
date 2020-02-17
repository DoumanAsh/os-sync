//! Wrappers over OS sync primitives

#![no_std]
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

mod sem;
pub use sem::{Sem, Semaphore, SemaphoreGuard};

pub mod mutex;
///Alias to [SemMutex](mutex/struct.SemMutex.html) with default [Semaphore](sem/trait.Semaphore.html) implementation
pub type SemMutex = mutex::SemMutex<Sem>;

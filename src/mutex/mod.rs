//! Mutex implementations

mod sem;
pub use sem::Mutex as SemMutex;
pub use sem::MutexGuard as SemMutexGuard;

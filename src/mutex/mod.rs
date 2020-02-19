//! Mutex implementations

mod sem;
pub use sem::Mutex as SemMutex;

///Token passed by [MutexGuard](struct.MutexGuard.html)
pub struct GuardToken {
}

///Describes Mutex interface
pub trait Mutex: Sized {
    ///Creates new instance
    ///
    ///Returns if `Semaphore` is successfully created.
    fn new() -> Option<Self>;

    ///Acquires lock, returning guard that unlocks self on drop.
    ///
    ///If lock is already acquired, it blocks until mutex is unlocked
    fn lock(&self) -> MutexGuard<'_, Self>;

    ///Attempts to acquire lock, returning guard that unlocks self on drop.
    ///
    ///If lock is already acquired, it returns `None`
    fn try_lock(&self) -> Option<MutexGuard<'_, Self>>;

    ///Tells how to perform unlock.
    ///
    ///Method implementation should be safe, but is allowed to mis-behave when invoked without
    ///prior `lock`
    fn unlock(&self, token: GuardToken);
}

///Guard, created by locking Mutex.
pub struct MutexGuard<'a, T: Mutex> {
    mutex: &'a T
}

impl<T: Mutex> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.unlock(GuardToken {});
    }
}

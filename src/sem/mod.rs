//!Semaphore primitive

#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
mod posix;
#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
pub use posix::Sem;

#[cfg(windows)]
mod win32;
#[cfg(windows)]
pub use win32::Sem;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod mac;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use mac::Sem;

///Describes Semaphore interface
///
///This primitive provides access to single integer that can be decremented using post
///and incremented using wait
pub trait Semaphore: Sized {
    ///Creates new instance, returning None on inability to do so.
    ///
    ///`init` is initial value for the semaphore
    fn new(init: u32) -> Option<Self>;

    ///Decrements self, returning immediately if it was signaled.
    ///
    ///Otherwise awaits for `post`
    fn wait(&self);

    ///Attempts to decrement self, returning whether self was signaled or not.
    ///
    ///Returns `true` if self was signaled
    ///
    ///Returns `false` otherwise
    fn try_wait(&self) -> bool;

    ///Attempts to decrement self within provided time, returning whether self was signaled or not.
    ///
    ///Returns `true` if self was signaled within specified `timeout`
    ///
    ///Returns `false` otherwise
    fn wait_timeout(&self, timeout: core::time::Duration) -> bool;

    ///Increments self
    ///
    ///When self becomes greater than zero, waiter shall be woken and result is `true`
    fn post(&self) -> bool;

    ///Gets semaphore's guard, which post on drop.
    ///
    ///Before guard is created, function will await for semaphore to get decremented.
    fn lock(&self) -> SemaphoreGuard<'_, Self> {
        self.wait();
        SemaphoreGuard {
            sem: self
        }
    }

    ///Attempts to acquire semaphore's guard, which post on drop.
    ///
    ///If semaphore cannot be decremented at the current moment, returns `None`
    fn try_lock(&self) -> Option<SemaphoreGuard<'_, Self>> {
        match self.try_wait() {
            true => Some(SemaphoreGuard {
                sem: self,
            }),
            false => None,
        }
    }
}

///[Semaphore](trait.Semaphore.html) guard
///
///Increments(post) semaphore on drop.
pub struct SemaphoreGuard<'a, T: Semaphore> {
    sem: &'a T,
}

impl<T: Semaphore> Drop for SemaphoreGuard<'_, T> {
    fn drop(&mut self) {
        self.sem.post();
    }
}

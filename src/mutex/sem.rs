use crate::sem::Semaphore;

use core::sync::atomic::{AtomicU32, Ordering};

///Semaphore based implementation, often called Benaphore
///
///Note that it is not recursive
pub struct Mutex<T> {
    sem: T,
    count: AtomicU32,
}

impl<T: Semaphore> Mutex<T> {
    #[inline]
    ///Creates new instance
    ///
    ///Returns if `Semaphore` is successfully created.
    pub fn new() -> Option<Self> {
        Some(Self {
            sem: T::new(0)?,
            count: AtomicU32::new(0),
        })
    }

    #[inline]
    ///Acquires lock, returning guard that unlocks self on drop.
    ///
    ///If lock is already acquired, it blocks until mutex is unlocked
    pub fn lock(&self) -> MutexGuard<'_, T> {
        if self.count.fetch_add(1, Ordering::AcqRel) > 0 {
            self.sem.wait();
        }

        MutexGuard {
            mutex: self,
        }
    }

    ///Attempts to acquire lock, returning guard that unlocks self on drop.
    ///
    ///If lock is already acquired, it returns `None`
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self.count.compare_and_swap(0, 1, Ordering::AcqRel) > 0 {
            None
        } else {
            Some(MutexGuard {
                mutex: self,
            })
        }
    }

    fn unlock(&self) {
        let old_count = self.count.fetch_sub(1, Ordering::AcqRel);

        //This is not possible for user as he only uses guard
        debug_assert_ne!(old_count, 0);

        self.sem.signal()
    }
}

///Guard, created by locking Mutex.
pub struct MutexGuard<'a, T: Semaphore> {
    mutex: &'a Mutex<T>
}

impl<T: Semaphore> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

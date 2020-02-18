use crate::sem::Semaphore;

use core::sync::atomic::{AtomicU32, Ordering};

///Semaphore based implementation, often called Benaphore
///
///Note that it is not recursive
pub struct Mutex<T> {
    sem: T,
    count: AtomicU32,
}

impl<T: Semaphore> super::Mutex for Mutex<T> {
    #[inline]
    fn new() -> Option<Self> {
        Some(Self {
            sem: T::new(0)?,
            count: AtomicU32::new(0),
        })
    }

    #[inline]
    fn lock(&self) -> super::MutexGuard<'_, Self> {
        if self.count.fetch_add(1, Ordering::AcqRel) > 0 {
            self.sem.wait();
        }

        super::MutexGuard {
            mutex: self,
        }
    }

    fn try_lock(&self) -> Option<super::MutexGuard<'_, Self>> {
        if self.count.compare_and_swap(0, 1, Ordering::AcqRel) > 0 {
            None
        } else {
            Some(super::MutexGuard {
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

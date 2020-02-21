use crate::sem::{Semaphore, Sem};

use core::ptr;
use core::sync::atomic::{AtomicU8, AtomicU32, AtomicPtr, Ordering};

const INCOMPLETE: u8 = 0x0;
const RUNNING: u8 = 0x1;
const COMPLETE: u8 = 0x2;
const FAIL: u8 = 0x4;

const FAIL_MSG: &str = "Unable to initialize semaphore";

struct StateGuard<'a> {
    state: &'a AtomicU8,
}

impl<'a> Drop for StateGuard<'a> {
    fn drop(&mut self) {
        self.state.store(COMPLETE, Ordering::Release);
    }
}

struct SemGuard {
    sem: Sem,
    waiting: AtomicU32,
}

impl SemGuard {
    fn wait(&self) {
        self.waiting.fetch_add(1, Ordering::Release);
        self.sem.wait();
    }
}

impl Drop for SemGuard {
    fn drop(&mut self) {
        for _ in 0..self.waiting.load(Ordering::Acquire) {
            self.sem.signal();
        }
    }
}

///A synchronization primitive which can be used to run a one-time global initialization.
pub struct Once {
    sem: AtomicPtr<SemGuard>,
    state: AtomicU8,
}

impl Once {
    ///Creates new instance
    pub const fn new() -> Self {
        Self {
            sem: AtomicPtr::new(ptr::null_mut()),
            state: AtomicU8::new(INCOMPLETE),
        }
    }

    ///Invokes provided closure once.
    ///
    ///Note that if function panics, `Once` is considered finished.
    pub fn call_once<F: FnOnce()>(&self, cb: F) {
        if self.is_completed() {
            return;
        }

        let mut cb = Some(cb);
        self.call_inner(move || match cb.take() {
            Some(cb) => cb(),
            None => unreach!()
        });
    }

    #[inline]
    ///Returns whether `Once` completed
    pub fn is_completed(&self) -> bool {
        self.state.load(Ordering::Acquire) == COMPLETE
    }

    #[cold]
    fn call_inner_fail(&self) -> ! {
        self.state.store(FAIL, Ordering::Acquire);
        panic!(FAIL_MSG)
    }

    #[cold]
    fn call_inner<F: FnMut()>(&self, mut cb: F) {
        loop {
            match self.state.load(Ordering::Acquire) {
                COMPLETE => break,
                FAIL => panic!(FAIL_MSG),
                INCOMPLETE => {
                    if INCOMPLETE != self.state.compare_and_swap(INCOMPLETE, RUNNING, Ordering::Acquire) {
                        continue;
                    }

                    let sem = match Sem::new(0) {
                        Some(sem) => sem,
                        None => self.call_inner_fail(),
                    };

                    let mut sem_guard = SemGuard {
                        sem,
                        waiting: AtomicU32::new(0),
                    };
                    self.sem.store(&mut sem_guard as *mut _, Ordering::Release);

                    //We should update state first, then free awaiting threads
                    let _state_guard = StateGuard {
                        state: &self.state
                    };

                    cb();

                },
                state => {
                    debug_assert_eq!(state, RUNNING);

                    let mut sem = self.sem.load(Ordering::Acquire);
                    while sem.is_null() {
                        if self.state.load(Ordering::Acquire) == FAIL {
                            panic!(FAIL_MSG);
                        }

                        sem = self.sem.load(Ordering::Acquire);
                        core::sync::atomic::spin_loop_hint();
                    }

                    if self.state.load(Ordering::Acquire) != RUNNING {
                        unsafe {
                            (*sem).wait()
                        }
                    }
                },
            }
        }
    }
}

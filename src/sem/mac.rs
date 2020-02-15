use core::ffi::c_void;
use core::mem;

#[repr(C)]
struct TimeSpec {
    tv_sec: libc::c_uint,
    tv_nsec: libc::c_int,
}

const KERN_OPERATION_TIMED_OUT: libc::c_int = 49;
const SYNC_POLICY_FIFO: libc::c_int = 0;

extern "C" {
    static mach_task_self_: libc::c_uint;

    //typedef struct semaphore *semaphore_t;
    //Function takes semaphore_t*
    fn semaphore_create(task: libc::c_uint, semaphore: *mut *mut c_void, policy: libc::c_int, value: libc::c_int) -> libc::c_int;
    fn semaphore_signal(semaphore: *mut c_void) -> libc::c_int;
    fn semaphore_wait(semaphore: *mut c_void) -> libc::c_int;
    fn semaphore_timedwait(semaphore: *mut c_void, timeout: TimeSpec) -> libc::c_int;
    fn semaphore_destroy(task: libc::c_uint, semaphore: *mut c_void) -> libc::c_int;
}

///MacOS semaphore based on mach API
///
///Due to limitation of mach API, post always returns `true`
pub struct Sem {
    handle: *mut c_void,
}

impl super::Semaphore for Sem {
    fn new(init: u32) -> Option<Self> {
        let mut handle = mem::MaybeUninit::uninit();

        let res = unsafe {
            semaphore_create(mach_task_self_, handle.as_mut_ptr(), SYNC_POLICY_FIFO, init as libc::c_int)
        };

        match res {
            0 => Some(Self {
                handle: unsafe { handle.assume_init() },
            }),
            _ => None,
        }
    }

    fn wait(&self) {
        let result = unsafe {
            semaphore_wait(self.handle)
        };

        debug_assert_eq!(result, 0, "semaphore_wait() failed");
    }

    fn try_wait(&self) -> bool {
        let result = unsafe {
            semaphore_timedwait(self.handle, mem::MaybeUninit::zeroed().assume_init())
        };

        debug_assert!(result == 0 || result == KERN_OPERATION_TIMED_OUT, "semaphore_timedwait() failed");
        result == 0
    }

    fn post(&self) -> bool {
        let res = unsafe {
            semaphore_signal(self.handle)
        };

        debug_assert_eq!(res, 0);
        true
    }
}

impl Drop for Sem {
    fn drop(&mut self) {
        unsafe {
            semaphore_destroy(mach_task_self_, self.handle);
        }
    }
}

unsafe impl Send for Sem {}
unsafe impl Sync for Sem {}

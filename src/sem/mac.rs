use core::ffi::c_void;

#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
type c_long = i64;
#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
type c_long = i32;

const DISPATCH_TIME_NOW: u64 = 0;
const DISPATCH_TIME_FOREVER: u64 = !0;

#[cfg_attr(any(target_os = "macos", target_os = "ios"), link(name = "System", kind = "dylib"))]
#[cfg_attr(not(any(target_os = "macos", target_os = "ios")), link(name = "dispatch", kind = "dylib"))]
extern {
    fn dispatch_semaphore_create(value: c_long) -> *mut c_void;
    fn dispatch_semaphore_signal(object: *mut c_void) -> c_long;
    fn dispatch_semaphore_wait(object: *mut c_void, timeout: u64) -> c_long;
    fn dispatch_release(object: *mut c_void);
}

///Apple dispatch source based Semaphore
///
///This implementation has particular drawback of disallowing to drop when current count is less
///than initial.
///
///In practice it means that unexpected drop of it would cause abort
pub struct Sem {
    handle: *mut c_void,
}

impl super::Semaphore for Sem {
    fn new(init: u32) -> Option<Self> {
        let handle = unsafe {
            dispatch_semaphore_create(init as c_long)
        };

        if handle.is_null() {
            return None;
        } else {
            Some(Self {
                handle
            })
        }
    }

    fn wait(&self) {
        let result = unsafe {
            dispatch_semaphore_wait(self.handle, DISPATCH_TIME_FOREVER)
        };

        debug_assert_eq!(result, 0, "dispatch_semaphore_wait() failed");
    }

    fn try_wait(&self) -> bool {
        let result = unsafe {
            dispatch_semaphore_wait(self.handle, DISPATCH_TIME_NOW)
        };

        result == 0
    }

    fn post(&self) -> Option<bool> {
        unsafe {
            Some(dispatch_semaphore_signal(self.handle) != 0)
        }
    }
}

impl Drop for Sem {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(self.handle);
        }
    }
}

unsafe impl Send for Sem {}
unsafe impl Sync for Sem {}

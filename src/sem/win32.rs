use core::ptr;
use core::ffi::c_void;

const WAIT_OBJECT_0: u32 = 0;
const WAIT_TIMEOUT: u32 = 0x00000102;
const INFINITE: u32 = 0xFFFFFFFF;

extern "system" {
    fn CloseHandle(handle: *mut c_void) -> i32;
    fn CreateSemaphoreW(attrs: *mut c_void, initial: i32, max: i32, name: *const u16) -> *mut c_void;
    fn WaitForSingleObject(handle: *mut c_void, timeout_ms: u32) -> u32;
    fn ReleaseSemaphore(handle: *mut c_void, increment: i32, previous_increment: *mut i32) -> i32;
}

///Windows implementation of Semaphore
pub struct Sem {
    handle: *mut c_void
}

impl super::Semaphore for Sem {
    fn new(init: u32) -> Option<Self> {
        let handle = unsafe {
            CreateSemaphoreW(ptr::null_mut(), init as i32, i32::max_value(), ptr::null())
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
            WaitForSingleObject(self.handle, INFINITE)
        };

        match result {
            WAIT_OBJECT_0 => (),
            //We cannot really timeout when there is no timeout
            other => panic!("Unexpected result: {}", other),
        }
    }

    fn try_wait(&self) -> bool {
        let result = unsafe {
            WaitForSingleObject(self.handle, 0)
        };

        match result {
            WAIT_OBJECT_0 => true,
            WAIT_TIMEOUT => false,
            other => panic!("Unexpected result: {}", other),
        }
    }

    fn post(&self) -> Option<bool> {
        let mut prev = 0;
        let result = unsafe {
            ReleaseSemaphore(self.handle, 1, &mut prev)
        };

        if result != 0 {
            Some(prev == 0)
        } else {
            None
        }
    }
}

impl Drop for Sem {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

unsafe impl Send for Sem {}
unsafe impl Sync for Sem {}

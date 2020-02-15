use os_sync::{Sem, Semaphore};

#[test]
fn should_return_when_signaled() {
    let sem = Sem::new(0).unwrap();

    assert!(sem.try_lock().is_none());
    assert!(sem.post());
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        assert!(!sem.post());
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        assert!(sem.post());
    }

    let _guard = sem.lock();
    assert!(sem.try_wait());
    assert!(!sem.try_wait());

    drop(_guard);
    assert!(sem.try_wait());
}

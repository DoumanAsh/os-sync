use os_sync::{Sem, Semaphore};

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
#[test]
fn should_return_when_signaled() {
    let sem = Sem::new(0).unwrap();

    assert!(!sem.try_wait());
    assert!(sem.post().unwrap());
    assert!(!sem.post().unwrap());
    sem.wait();
    assert!(sem.try_wait());
    assert!(!sem.try_wait());
}

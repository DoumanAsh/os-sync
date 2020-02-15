use os_sync::{Sem, Semaphore};

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

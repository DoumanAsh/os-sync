use os_sync::SemMutex;
use core::sync::atomic::{AtomicBool, Ordering};

#[test]
fn should_lock_sem_mutex_without_contest() {
    let mutex = SemMutex::new().unwrap();

    {
        let _lock = mutex.lock();
    }

    {
        let _lock = mutex.try_lock().unwrap();
    }
}

#[test]
fn should_lock_sem_mutex_with_contest() {
    let flag = std::sync::Arc::new(AtomicBool::new(false));
    let flag_clone = flag.clone();
    let mutex = std::sync::Arc::new(SemMutex::new().unwrap());
    let mutex_clone = mutex.clone();

    let join = std::thread::spawn(move || {
        let _lock = mutex_clone.lock();
        flag_clone.store(true, Ordering::Relaxed);
    });

    let _lock = mutex.lock();
    assert!(!flag.load(Ordering::Relaxed));
    drop(_lock);

    join.join().unwrap();
    assert!(flag.load(Ordering::Relaxed));
}

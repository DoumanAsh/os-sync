use os_sync::Once;
use core::sync::atomic::{Ordering, AtomicU8};

#[test]
fn should_call_once1() {
    static COUNTER: AtomicU8 = AtomicU8::new(0);

    fn routine() {
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            COUNTER.fetch_add(1, Ordering::SeqCst);
        });
    }

    let mut threads = Vec::new();

    for _ in 0..8 {
        threads.push(std::thread::spawn(routine));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(COUNTER.load(Ordering::Relaxed), 1);
}


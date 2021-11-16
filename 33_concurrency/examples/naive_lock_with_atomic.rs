use std::{
    cell::RefCell,
    fmt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

struct Lock<T> {
    locked: AtomicBool,
    data: RefCell<T>,
}

impl<T> fmt::Debug for Lock<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}

// SAFETY: 我们确信 Lock<T> 很安全，可以在多个线程中共享
unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: RefCell::new(data),
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self, op: impl FnOnce(&mut T)) {
        // 如果没拿到锁，就一直 spin
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // 性能优化：compare_exchange 需要独占访问，当拿不到锁时，我们
            // 先不停检测 locked 的状态，直到其 unlocked 后，再尝试拿锁
            while self.locked.load(Ordering::Relaxed) {}
        } // **1

        // 已经拿到并加锁，开始干活
        op(&mut self.data.borrow_mut()); // **3

        // 解锁
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let data = Arc::new(Lock::new(0));

    let data1 = data.clone();
    let t1 = thread::spawn(move || {
        data1.lock(|v| *v += 10);
    });

    let data2 = data.clone();
    let t2 = thread::spawn(move || {
        data2.lock(|v| *v *= 10);
    });
    t1.join().unwrap();
    t2.join().unwrap();

    println!("data: {:?}", data);
}

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[allow(clippy::all)]
fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;

        // 通过离开作用域来释放锁，否则主线程会一直阻塞在获取锁上
        {
            let mut started = lock.lock().unwrap();
            *started = true;
        }

        eprintln!("I'm a happy worker!");
        // 通知主线程
        cvar.notify_one();
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("working...");
        }
    });

    // 等待工作线程的通知
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    eprintln!("Worker started!");

    // 等待 worker 线程
    thread::sleep(Duration::from_secs(3600));
}

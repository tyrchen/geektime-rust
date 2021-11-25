use futures::{executor::block_on, Future};
use std::thread::{self, JoinHandle};

fn main() {
    // thread 可以处理异步任务
    let t1 = thread_async();
    // Future 可以处理异步任务
    let t2 = task_async();

    // 线程一旦启动，就开始执行，这里的 join 不过是等待结果
    let r1 = t1.join().unwrap();
    // Future 需要显式地 await 才能开始执行
    let r2 = block_on(async move { t2.await });

    assert_eq!(r1, r2);
}

fn thread_async() -> JoinHandle<usize> {
    thread::spawn(move || {
        println!("hello thread!");
        42
    })
}

#[allow(clippy::all)]
fn task_async() -> impl Future<Output = usize> {
    async {
        println!("hello async!");
        42
    }
}

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

// Rc 既不是 Send，也不是 Sync
#[allow(dead_code, unused_variables)]
fn rc_is_not_send_and_sync() {
    let a = Rc::new(1);
    let b = a.clone();
    let c = a.clone();

    println!("{:?} {:?} {:?}", a, b, c);

    // 无法编译通过
    // thread::spawn(move || {
    //     println!("c= {:?}", c);
    // });
}

#[allow(dead_code)]
fn refcell_is_send() {
    let a = RefCell::new(1);
    thread::spawn(move || {
        println!("a= {:?}", a);
    });
}

// RefCell 现在有多个 Arc 持有它，虽然 Arc 是 Send/Sync，但 RefCell 不是 Sync
#[allow(dead_code, unused_variables)]
fn refcell_is_not_sync() {
    let a = Arc::new(RefCell::new(1));
    let b = a.clone();
    let c = a.clone();

    println!("{:?} {:?} {:?}", a, b, c);

    // 无法编译通过
    // thread::spawn(move || {
    //     println!("c= {:?}", c);
    // });
}

// Arc<Mutext<T>> 可以多线程共享且修改数据
#[allow(dead_code)]
fn arc_mutext_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();
    let handle = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 1;
    });

    {
        let mut g = b.lock().unwrap();
        *g += 1;
    }

    handle.join().unwrap();
    println!("a= {:?}", a);
}

// 无法编译通过
// fn mutex_guard_is_not_send() {
//     let mutex = Mutex::new(1);
//     let guard = mutex.lock().unwrap();
//     thread::spawn(|| {
//         println!("data= {:?}", guard);
//     });

//     thread::spawn(move || {
//         println!("data= {:?}", guard);
//     });
// }

fn main() {}

use std::sync::Mutex;

fn main() {
    //
    let m = Mutex::new(Mutex::new(1));
    let g = m.lock().unwrap();
    {
        rayon::join(
            || {
                let mut g1 = g.lock().unwrap();
                *g1 += 1;
                println!("Thread 1: {:?}", *g1);
            },
            || {
                let mut g1 = g.lock().unwrap();
                *g1 += 1;
                println!("Thread 1: {:?}", *g1);
            },
        );
    }
}

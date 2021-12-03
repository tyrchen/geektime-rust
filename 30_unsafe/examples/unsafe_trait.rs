/// # Safety
/// 实现这个 trait 的开发者要保证实现是内存安全的
unsafe trait Foo {
    fn foo(&self);
}

trait Bar {
    // 调用这个函数的人要保证调用是安全的
    unsafe fn bar(&self);
}

struct Nonsense;

/// # Safety
/// example
unsafe impl Foo for Nonsense {
    fn foo(&self) {
        println!("foo!");
    }
}

impl Bar for Nonsense {
    unsafe fn bar(&self) {
        println!("bar!");
    }
}

fn main() {
    let nonsense = Nonsense;
    // 调用者无需关心 safety
    nonsense.foo();

    // 调用者需要为 safety 负责
    unsafe { nonsense.bar() };
}

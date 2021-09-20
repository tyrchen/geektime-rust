use std::fmt::{Debug, Display};
use std::mem::transmute;

fn main() {
    let s = String::from("hello world!");
    let s1 = String::from("goodbye world!");
    // Display / Debug trait object for s
    let w1: &dyn Display = &s;
    let w2: &dyn Debug = &s;

    // Display / Debug trait object for s1
    let w3: &dyn Display = &s1;
    let w4: &dyn Debug = &s1;

    // 强行把 triat object 转换成两个地址 (usize, usize)
    // 这是不安全的，所以是 unsafe
    let (addr1, vtable1): (usize, usize) = unsafe { transmute(w1) };
    let (addr2, vtable2): (usize, usize) = unsafe { transmute(w2) };
    let (addr3, vtable3): (usize, usize) = unsafe { transmute(w3) };
    let (addr4, vtable4): (usize, usize) = unsafe { transmute(w4) };

    // s 和 s1 在栈上的地址，以及 main 在 TEXT 段的地址
    println!(
        "s: {:p}, s1: {:p}, main(): {:p}",
        &s, &s1, main as *const ()
    );
    // trait object(s / Display) 的 ptr 地址和 vtable 地址
    println!("addr1: 0x{:x}, vtable1: 0x{:x}", addr1, vtable1);
    // trait object(s / Debug) 的 ptr 地址和 vtable 地址
    println!("addr2: 0x{:x}, vtable2: 0x{:x}", addr2, vtable2);

    // trait object(s1 / Display) 的 ptr 地址和 vtable 地址
    println!("addr3: 0x{:x}, vtable3: 0x{:x}", addr3, vtable3);

    // trait object(s1 / Display) 的 ptr 地址和 vtable 地址
    println!("addr4: 0x{:x}, vtable4: 0x{:x}", addr4, vtable4);

    // 指向同一个数据的 trait object 其 ptr 地址相同
    assert_eq!(addr1, addr2);
    assert_eq!(addr3, addr4);

    // 指向同一种类型的同一个 trait 的 vtable 地址相同
    // 这里都是 String + Display
    assert_eq!(vtable1, vtable3);
    // 这里都是 String + Debug
    assert_eq!(vtable2, vtable4);
}

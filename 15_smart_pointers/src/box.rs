// #![feature(dropck_eyepatch)]

// struct MyBox<T>(Box<T>);

// unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
//     fn drop(&mut self) {
//         todo!();
//     }
// }

fn main() {
    // 在堆上分配 16M 内存，但它会现在栈上出现，再移动到堆上
    let boxed = Box::new([0u8; 1 << 24]);
    println!("len: {}", boxed.len());
}

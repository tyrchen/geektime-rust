fn main() {
    let mut age = 18;

    // 不可变引用
    let r1 = &age as *const i32;
    // 可变引用
    let r2 = &mut age as *mut i32;

    // 使用裸指针，可以绕过 immutable / mutable borrow rule

    // 然而，对指针解引用需要使用 unsafe
    unsafe {
        println!("r1: {}, r2: {}", *r1, *r2);
    }
}

// fn immutable_mutable_cant_coexist() {
//     let mut age = 18;
//     let r1 = &age;
//     // 编译错误
//     let r2 = &mut age;

//     println!("r1: {}, r2: {}", *r1, *r2);
// }

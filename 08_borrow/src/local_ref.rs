fn main() {
    let r = local_ref();
    println!("r: {:p}", r);
}

#[allow(unused_variables)]
fn local_ref<'a>() -> &'a i32 {
    let a = 42;
    // 不能返回对局部变量 a 的引用
    // &a
    todo!();
}

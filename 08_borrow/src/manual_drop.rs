use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
struct MyString(String);

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        MyString(s.to_string())
    }
}

impl Drop for MyString {
    fn drop(&mut self) {
        println!("Going to drop: {}", self.0);
    }
}

impl Deref for MyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MyString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    // 使用 ManuallyDrop 封装数据结构使其不进行自动 drop
    let mut s = ManuallyDrop::new(MyString::from("Hello World!"));

    // ManuallyDrop 使用了 Deref trait 指向 T，所以可以当 MyString 使用，MyString 又可以当 String 用
    s.truncate(5);
    println!("s: {:?}", s);

    // 如果没有这句，s 不会在 scope 结束时被自动 drop（你可以注掉试一下）
    // 如果我们想让它可以自动 drop，可以用 into_inner
    let _: MyString = ManuallyDrop::into_inner(s);
}

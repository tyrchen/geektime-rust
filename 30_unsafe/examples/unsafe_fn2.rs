use std::str;

fn main() {
    let data = b"hello world!";

    // Safety: 从上下文中我们可以保证 data 只包含合法的 utf8 字符
    let s = unsafe { str::from_utf8_unchecked(data) };

    println!("s: {}", s);
}

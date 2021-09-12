use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("hello", "world");
    let key = "hello1";

    // 按照之前的说法，这段代码无法编译通过，因为同一个 scope 下不能有两个可变引用
    // 但因为 RFC2094 non-lexical lifetimes，Rust 编译器可以处理这个场景，
    // 因为当 None 时，map.get_mut() 的引用实际已经结束
    match map.get_mut(key) /* <----- 可变引用的生命周期一直持续到 match 结果 */ {
        Some(v) => do_something(v),
        None => {
            map.insert(key, "tyr"); // <--- 这里又获得了一个可变引用
        }
    }

    let s = std::sync::Arc::new(String::from("hello world"));
    println!("{:?}", s);
}

fn do_something(_v: &mut &str) {
    todo!()
}

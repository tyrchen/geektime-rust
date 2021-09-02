fn main() {
    let name = "Tyr".to_string();
    // 对于编译错误，Rust 编译器会给出详细的错误原因，并给出建议的修复方法
    std::thread::spawn(|| {
        println!("hello1 {}", name);
    });
}

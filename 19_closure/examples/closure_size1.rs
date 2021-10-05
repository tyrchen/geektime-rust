fn main() {
    let name = String::from("Tyr");
    let data = vec!["Rust", "Elixir", "Javascript"];
    let v = &data[..];
    let i = 1u8;
    let c = move || {
        println!("i: {:?}", i);
        println!("v: {:?}, name: {:?}", v, name.clone());
    };
    c();
    println!("size of c: {}", std::mem::size_of_val(&c));

    // 请问在这里，还能访问 name 么？为什么？
}

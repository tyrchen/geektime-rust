#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

fn main() {
    let dev = Developer {
        name: "Tyr".to_string(),
        age: 18,
        lang: Language::Rust,
    };
    let dev1 = dev.clone();
    println!("dev: {:?}, addr of name: {:p}", dev, dev.name.as_str());
    println!("dev1: {:?}, addr of name: {:p}", dev1, dev1.name.as_str());
}

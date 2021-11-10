uniffi_macros::include_scaffolding!("math");

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn hello(name: &str) -> String {
    format!("hello {}!", name)
}

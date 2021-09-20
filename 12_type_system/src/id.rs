fn id<T>(x: T) -> T {
    x
}

fn main() {
    let int = id(42);
    let string = id("Tyr");
    println!("{}, {}", int, string);
}

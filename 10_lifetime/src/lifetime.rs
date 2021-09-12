fn main() {
    let s1 = String::from("Lindsey");
    let s2 = String::from("Rosie");

    let result = max(&s1, &s2);

    println!("bigger one: {}", result);

    let result = get_max(s1);
    println!("bigger one: {}", result);
}

fn get_max(s1: &str) -> &str {
    max(s1, "Cynthia")
}

// 这段代码无法编译通过
fn max(s1: &str, s2: &str) -> &str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

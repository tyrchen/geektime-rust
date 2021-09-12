fn main() {
    let s1 = "Lindsey";
    let s2 = String::from("Rosie");

    let result = max(s1, &s2);

    println!("bigger one: {}", result);
}

fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

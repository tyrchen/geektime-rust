fn main() {
    let s1 = String::from("Lindsey");
    let result;
    {
        let s2 = String::from("Rosie");
        // s2 生命周期不够长
        result = max(&s1, &s2);
    }

    println!("bigger one: {}", result);
}

fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

trait Print {
    fn print(self);
}

// lifetime 也不能有不同的实现
// impl<'a> Print for &'a str {
//     fn print(self) {
//         println!("Arbitrary str: {}", self);
//     }
// }

impl Print for &'static str {
    fn print(self) {
        println!("'static str: {}", self);
    }
}

// lifetime 不会单体化，所以这个不工作
// fn print_str<'a>(s: &'a str) {
//     s.print()
// }

fn main() {
    let s = "hello, world!";
    s.print();
    // print_str(s);
}

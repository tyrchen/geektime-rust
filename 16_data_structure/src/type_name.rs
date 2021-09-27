pub trait TypeName {
    fn type_name(&self) -> &'static str;
}

impl<T> TypeName for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

fn main() {
    let s = String::from("hello");
    let s1 = &s;
    let s2 = s.as_str();
    let s3 = &s[..];
    println!(
        "s: {}, s1: {}, s2: {}, s3: {}",
        s.type_name(),
        s1.type_name(),
        s2.type_name(),
        s3.type_name()
    );

    let v = vec![1, 2, 3, 4];
    let v1 = &v;
    let v2 = v.as_slice();
    let v3 = &v[..];
    let v4 = v.clone().into_boxed_slice();

    println!(
        "v: {}, v1: {}, v2: {}, v3: {}, v4: {}",
        v.type_name(),
        v1.type_name(),
        v2.type_name(),
        v3.type_name(),
        v4.type_name()
    );
}

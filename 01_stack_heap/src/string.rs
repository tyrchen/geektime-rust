fn main() {
    let data: String = "hello".into();

    let s1: &str = &data;
    let s2: &str = &data;
    let s3: &str = &data;

    dbg!(&s1 as *const _);
    dbg!(&s2 as *const _);
    dbg!(&s3 as *const _);

    dbg!(s1.as_bytes() as *const _);
    dbg!(s2.as_bytes() as *const _);
    dbg!(s3.as_bytes() as *const _);
}

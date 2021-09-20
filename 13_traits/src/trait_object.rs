use std::{fs::File, io::Write};
fn main() {
    let mut f = File::create("/tmp/test_write_trait").unwrap();
    let w: &mut dyn Write = &mut f;
    w.write_all(b"hello ").unwrap();

    // 无法在 trait object 里调用使用了 Self 的函数
    // let w1 = w.by_ref();
    // w1.write_all(b"world").unwrap();
}

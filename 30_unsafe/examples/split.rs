use std::{slice::from_raw_parts_mut, str::from_utf8_unchecked_mut};

fn main() {
    let mut s = "我爱你！中国".to_string();
    let r = s.as_mut();

    if let Some((s1, s2)) = split(r, '！') {
        println!("s1: {}, s2: {}", s1, s2);
    }
}

fn split(s: &str, sep: char) -> Option<(&str, &str)> {
    let pos = s.find(sep);

    pos.map(|pos| {
        let len = s.len();
        let sep_len = sep.len_utf8();

        // SAFETY: pos 是 find 得到的，它位于字符的边界处，同样 pos + sep_len 也是如此
        // 所以以下代码是安全的
        unsafe { (s.get_unchecked(0..pos), s.get_unchecked(pos + sep_len..len)) }
    })
}

#[allow(dead_code)]
fn split_mut(s: &mut str, sep: char) -> Option<(&mut str, &mut str)> {
    let pos = s.find(sep);

    pos.map(|pos| {
        let ptr = s.as_mut_ptr();
        let len = s.len();
        let sep_len = sep.len_utf8();

        // SAFETY：s1 和 s2 不会产生重叠的部分，且 find 找到的 pos 是字符所以是安全的
        let s1 = unsafe { from_raw_parts_mut(ptr, pos) };
        let s2 = unsafe { from_raw_parts_mut(ptr.add(pos + sep_len), len - pos - sep_len) };

        // SAFETY：s1 和 s2 本就来自字符串
        unsafe { (from_utf8_unchecked_mut(s1), from_utf8_unchecked_mut(s2)) }
    })
}

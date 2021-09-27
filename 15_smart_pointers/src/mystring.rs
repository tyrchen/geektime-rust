use std::{fmt, ops::Deref, str};

const MINI_STRING_MAX_LEN: usize = 30;

// MyString 里，String 有 3 个 word，供 24 字节，所以它以 8 字节对齐
// 所以 enum 的 tag + padding 最少 8 字节，整个结构占 32 字节。
// MiniString 可以最多有 30 字节（再加上 1 字节长度和 1字节 tag），就是 32 字节.
struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    // 这里 new 接口不暴露出去，保证传入的 v 的字节长度小于等于 30
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        // 我们在拷贝内容时一定要要使用字符串的字节长度
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // 由于生成 MiniString 的接口是隐藏的，它只能来自字符串，所以下面这行是安全的
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
        // 也可以直接用 unsafe 版本
        // unsafe { str::from_utf8_unchecked(&self.data[..self.len as usize]) }
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 这里由于实现了 Deref trait，可以直接得到一个 &str 输出
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

// 实现 Deref 接口对两种不同的场景统一得到 &str
impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}

// impl From<&str> for MyString {
//     fn from(s: &str) -> Self {
//         match s.len() > MINI_STRING_MAX_LEN {
//             true => Self::Standard(s.to_owned()),
//             _ => Self::Inline(MiniString::new(s)),
//         }
//     }
// }

// impl From<String> for MyString {
//     fn from(s: String) -> Self {
//         match s.len() > MINI_STRING_MAX_LEN {
//             true => Self::Standard(s),
//             _ => Self::Inline(MiniString::new(s)),
//         }
//     }
// }

impl<T> From<T> for MyString
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        match s.as_ref().len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.as_ref().to_owned()),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl MyString {
    pub fn push_str(&mut self, s: &str) {
        match *self {
            MyString::Inline(ref mut v) => {
                let len = v.len as usize;
                let len1 = s.len();
                if len + len1 > MINI_STRING_MAX_LEN {
                    let mut owned = v.deref().to_string();
                    owned.push_str(s);
                    *self = MyString::Standard(owned);
                } else {
                    let total = len + len1;
                    v.data[len..len + len1].copy_from_slice(s.as_bytes());
                    v.len = total as u8;
                }
            }
            MyString::Standard(ref mut v) => v.push_str(s),
        }
    }
}

fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString {}, MiniString {}", len1, len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();

    // debug 输出
    println!("s1: {:?}, s2: {:?}", s1, s2);
    // display 输出
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );

    // MyString 可以使用一切 &str 接口，感谢 Rust 的自动 Deref
    assert!(s1.ends_with("world"));
    assert!(s2.starts_with('这'));

    let s = String::from("这是一个超过了三十个字节的很长很长的字符串");
    println!("s: {:p}", &*s);
    // From<T: AsRef<str>> 的实现会导致额外的复制
    let s3: MyString = s.into();
    println!("s3: {:p}", &*s3);

    let mut s4: MyString = "Hello Tyr! ".into();
    println!("s4: {:?}", s4);
    s4.push_str("这是一个超过了三十个字节的很长很长的字符串");
    println!("s4: {:?}", s4);
}

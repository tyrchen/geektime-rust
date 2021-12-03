use libc::c_char;
use std::{
    ffi::{CStr, CString},
    panic::catch_unwind,
    ptr,
};

// 使用 no_mangle 禁止函数名改编，这样其它语言可以通过 C ABI 调用这个函数
#[no_mangle]
pub extern "C" fn hello_world() -> *const c_char {
    // C String 以 "\0" 结尾，你可以把 "\0" 去掉看看会发生什么
    "hello world!\0".as_ptr() as *const c_char
}

/// # Safety
/// 这个函数是不安全的，别调！
#[allow(dead_code)]
#[no_mangle]
pub unsafe extern "C" fn hello_bad(name: *const c_char) -> *const c_char {
    // name 会不会是 NULL，是否是个合法的地址？
    // unwrap() 会导致 stack unwind，stack unwind 跨越 FFI 边界会导致未定义行为
    let s = CStr::from_ptr(name).to_str().unwrap();
    // 可以这样搞么？
    format!("hello {}!\0", s).as_ptr() as *const c_char
}

// 编译器会报警 str / String 不是 FFI
// #[no_mangle]
// pub extern "C" fn goodbye(name: &str) -> String {
//     format!("hello {}!", name)
// }

#[no_mangle]
pub extern "C" fn hello(name: *const c_char) -> *const c_char {
    if name.is_null() {
        return ptr::null();
    }

    let result = catch_unwind(|| {
        if let Ok(s) = unsafe { CStr::from_ptr(name).to_str() } {
            let result = format!("hello {}!", s);
            // 可以使用 unwrap，因为 result 不包含 \0
            let s = CString::new(result).unwrap();

            s.into_raw()
            // 相当于：
            // let p = s.as_ptr();
            // std::mem::forget(s);
            // p
        } else {
            ptr::null()
        }
    });

    match result {
        Ok(s) => s,
        Err(_) => ptr::null(),
    }
}

/// # Safety
/// 提供给 C 侧释放字符串指针，调用者需要保证指针来自 Rust
#[no_mangle]
pub unsafe extern "C" fn free_str(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_works() {
        let cstr = hello_world();
        let s = unsafe { CStr::from_ptr(cstr).to_str().unwrap() };
        assert_eq!(s, "hello world!");
    }

    #[test]
    fn hello_works() {
        let name = CStr::from_bytes_with_nul(b"Tyr\0").unwrap().as_ptr();
        let cstr = hello(name);
        println!("{:p}", cstr);
        let s = unsafe { CStr::from_ptr(cstr).to_str().unwrap() };
        assert_eq!(s, "hello Tyr!");
        // Rust 分配的内存，Rust 释放
        // cast *const 为 *mut 是一个不好的习惯，这里我只是为了演示 C/C++ 如何把指针回传，让 Rust 释放这个内存
        unsafe { free_str(cstr as *mut c_char) };
    }
}

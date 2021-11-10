fn main() {
    // 告诉 rustc 需要 link bzip2
    println!("cargo:rustc-link-lib=bz2");

    // 告诉 cargo 当 wrapper.h 变化时重新运行
    println!("cargo:rerun-if-changed=wrapper.h");

    // 配置 bindgen，并生成 Bindings 结构
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // 生成 Rust 代码
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Failed to write bindings");
}

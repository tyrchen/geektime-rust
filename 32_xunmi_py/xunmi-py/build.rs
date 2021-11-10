fn main() {
    // 只有 build.rs 更改后才重新编译
    println!("cargo:rerun-if-changed=build.rs");
    // 添加
    pyo3_build_config::add_extension_module_link_args();
}

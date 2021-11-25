use anyhow::Result;
use serde_yaml::Value;
use std::fs;

fn main() -> Result<()> {
    // 读取 Cargo.toml，IO 操作 1
    let content1 = fs::read_to_string("./Cargo.toml")?;
    // 读取 Cargo.lock，IO 操作 2
    let content2 = fs::read_to_string("./Cargo.lock")?;

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入 /tmp/Cargo.yml，IO 操作 3
    fs::write("/tmp/Cargo.yml", &yaml1)?;
    // 写入 /tmp/Cargo.lock，IO 操作 4
    fs::write("/tmp/Cargo.lock", &yaml2)?;

    // 打印
    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(content)?;
    Ok(serde_yaml::to_string(&value)?)
}

use anyhow::Result;
use serde_yaml::Value;
use tokio::{fs, try_join};

#[tokio::main]
async fn main() -> Result<()> {
    // 读取 Cargo.toml，IO 操作 1
    let f1 = fs::read_to_string("./Cargo.toml");
    // 读取 Cargo.lock，IO 操作 2
    let f2 = fs::read_to_string("./Cargo.lock");
    let (content1, content2) = try_join!(f1, f2)?;

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入 /tmp/Cargo.yml，IO 操作 3
    let f3 = fs::write("/tmp/Cargo.yml", &yaml1);
    // 写入 /tmp/Cargo.lock，IO 操作 4
    let f4 = fs::write("/tmp/Cargo.lock", &yaml2);
    try_join!(f3, f4)?;

    // 打印
    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(content)?;
    Ok(serde_yaml::to_string(&value)?)
}

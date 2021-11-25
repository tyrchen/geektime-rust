use anyhow::{anyhow, Result};
use serde_yaml::Value;
use std::{
    fs,
    thread::{self, JoinHandle},
};

/// 包装一下 JoinHandle，这样可以提供额外的方法
struct MyJoinHandle<T>(JoinHandle<Result<T>>);

impl<T> MyJoinHandle<T> {
    /// 等待 thread 执行完（类似 await）
    pub fn thread_await(self) -> Result<T> {
        self.0.join().map_err(|_| anyhow!("failed"))?
    }
}

fn main() -> Result<()> {
    // 读取 Cargo.toml，IO 操作 1
    let t1 = thread_read("./Cargo.toml");
    // 读取 Cargo.lock，IO 操作 2
    let t2 = thread_read("./Cargo.lock");

    let content1 = t1.thread_await()?;
    let content2 = t2.thread_await()?;

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入 /tmp/Cargo.yml，IO 操作 3
    let t3 = thread_write("/tmp/Cargo.yml", yaml1);
    // 写入 /tmp/Cargo.lock，IO 操作 4
    let t4 = thread_write("/tmp/Cargo.lock", yaml2);

    let yaml1 = t3.thread_await()?;
    let yaml2 = t4.thread_await()?;

    fs::write("/tmp/Cargo.yml", &yaml1)?;
    fs::write("/tmp/Cargo.lock", &yaml2)?;

    // 打印
    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())
}

fn thread_read(filename: &'static str) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        let s = fs::read_to_string(filename)?;
        Ok::<_, anyhow::Error>(s)
    });
    MyJoinHandle(handle)
}

fn thread_write(filename: &'static str, content: String) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        fs::write(filename, &content)?;
        Ok::<_, anyhow::Error>(content)
    });
    MyJoinHandle(handle)
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(content)?;
    Ok(serde_yaml::to_string(&value)?)
}

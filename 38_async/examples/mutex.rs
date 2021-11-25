use anyhow::Result;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

struct DB;

impl DB {
    // 假装在 commit 数据
    async fn commit(&mut self) -> Result<usize> {
        Ok(42)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db1 = Arc::new(Mutex::new(DB));
    let db2 = Arc::clone(&db1);

    tokio::spawn(async move {
        let mut db = db1.lock().await;
        // 因为拿到的 MutexGuard 要跨越 await，所以不能用 std::sync::Mutex
        // 只能用 tokio::sync::Mutex
        let affected = db.commit().await?;
        println!("db1: Total affected rows: {}", affected);
        Ok::<_, anyhow::Error>(())
    });

    tokio::spawn(async move {
        let mut db = db2.lock().await;
        let affected = db.commit().await?;
        println!("db2: Total affected rows: {}", affected);

        Ok::<_, anyhow::Error>(())
    });

    // 让两个 task 有机会执行完
    tokio::time::sleep(Duration::from_millis(1)).await;

    Ok(())
}

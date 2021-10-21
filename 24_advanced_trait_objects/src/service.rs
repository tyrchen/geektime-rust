use std::{error::Error, sync::Arc};

// 定义类型，让 KV server 里的 trait 可以被编译通过
pub type KvError = Box<dyn Error + Send + Sync>;
pub struct Value(i32);
pub struct Kvpair(i32, i32);

/// 对存储的抽象，我们不关心数据存在哪儿，但需要定义外界如何和存储打交道
pub trait Storage: Send + Sync + 'static {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

// 使用 trait object，不需要泛型参数，也不需要 ServiceInner 了
pub struct Service {
    pub store: Arc<dyn Storage>,
}

// impl 的代码略微简单一些
impl Service {
    pub fn new<S: Storage>(store: S) -> Self {
        Self {
            store: Arc::new(store),
        }
    }
}

// 实现 trait 时也不需要带着泛型参数
impl Clone for Service {
    fn clone(&self) -> Self {
        Self {
            store: Arc::clone(&self.store),
        }
    }
}

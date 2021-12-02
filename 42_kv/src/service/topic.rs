use dashmap::{DashMap, DashSet};
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use crate::{CommandResponse, KvError, Value};

/// topic 里最大存放的数据
const BROADCAST_CAPACITY: usize = 128;

/// 下一个 subscription id
static NEXT_ID: AtomicU32 = AtomicU32::new(1);

/// 获取下一个 subscription id
fn get_next_subscription_id() -> u32 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

pub trait Topic: Send + Sync + 'static {
    /// 订阅某个主题
    fn subscribe(self, name: String) -> mpsc::Receiver<Arc<CommandResponse>>;
    /// 取消对主题的订阅
    fn unsubscribe(self, name: String, id: u32) -> Result<u32, KvError>;
    /// 往主题里发布一个数据
    fn publish(self, name: String, value: Arc<CommandResponse>);
}

/// 用于主题发布和订阅的数据结构
#[derive(Default)]
pub struct Broadcaster {
    /// 所有的主题列表
    topics: DashMap<String, DashSet<u32>>,
    /// 所有的订阅列表
    subscriptions: DashMap<u32, mpsc::Sender<Arc<CommandResponse>>>,
}

impl Topic for Arc<Broadcaster> {
    fn subscribe(self, name: String) -> mpsc::Receiver<Arc<CommandResponse>> {
        let id = {
            let entry = self.topics.entry(name).or_default();
            let id = get_next_subscription_id();
            entry.value().insert(id);
            id
        };

        // 生成一个 mpsc channel
        let (tx, rx) = mpsc::channel(BROADCAST_CAPACITY);

        let v: Value = (id as i64).into();

        // 立刻发送 subscription id 到 rx
        let tx1 = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = tx1.send(Arc::new(v.into())).await {
                // TODO: 这个很小概率发生，但目前我们没有善后
                warn!("Failed to send subscription id: {}. Error: {:?}", id, e);
            }
        });

        // 把 tx 存入 subscription table
        self.subscriptions.insert(id, tx);
        debug!("Subscription {} is added", id);

        // 返回 rx 给网络处理的上下文
        rx
    }

    fn unsubscribe(self, name: String, id: u32) -> Result<u32, KvError> {
        match self.remove_subscription(name, id) {
            Some(id) => Ok(id),
            None => Err(KvError::NotFound(format!("subscription {}", id))),
        }
    }

    fn publish(self, name: String, value: Arc<CommandResponse>) {
        tokio::spawn(async move {
            let mut ids = vec![];
            if let Some(topic) = self.topics.get(&name) {
                // 复制整个 topic 下所有的 subscription id
                // 这里我们每个 id 是 u32，如果一个 topic 下有 10k 订阅，复制的成本
                // 也就是 40k 堆内存（外加一些控制结构），所以效率不算差
                // 这也是为什么我们用 NEXT_ID 来控制 subscription id 的生成

                let subscriptions = topic.value().clone();
                // 尽快释放锁
                drop(topic);

                // 循环发送
                for id in subscriptions.into_iter() {
                    if let Some(tx) = self.subscriptions.get(&id) {
                        if let Err(e) = tx.send(value.clone()).await {
                            warn!("Publish to {} failed! error: {:?}", id, e);
                            // client 中断连接
                            ids.push(id);
                        }
                    }
                }
            }

            for id in ids {
                self.remove_subscription(name.clone(), id);
            }
        });
    }
}

impl Broadcaster {
    pub fn remove_subscription(&self, name: String, id: u32) -> Option<u32> {
        if let Some(v) = self.topics.get_mut(&name) {
            // 在 topics 表里找到 topic 的 subscription id，删除
            v.remove(&id);

            // 如果这个 topic 为空，则也删除 topic
            if v.is_empty() {
                info!("Topic: {:?} is deleted", &name);
                drop(v);
                self.topics.remove(&name);
            }
        }

        debug!("Subscription {} is removed!", id);
        // 在 subscription 表中同样删除
        self.subscriptions.remove(&id).map(|(id, _)| id)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use tokio::sync::mpsc::Receiver;

    use crate::assert_res_ok;

    use super::*;

    #[tokio::test]
    async fn pub_sub_should_work() {
        let b = Arc::new(Broadcaster::default());
        let lobby = "lobby".to_string();

        // subscribe
        let mut stream1 = b.clone().subscribe(lobby.clone());
        let mut stream2 = b.clone().subscribe(lobby.clone());

        // publish
        let v: Value = "hello".into();
        b.clone().publish(lobby.clone(), Arc::new(v.clone().into()));

        // subscribers 应该能收到 publish 的数据
        let id1 = get_id(&mut stream1).await;
        let id2 = get_id(&mut stream2).await;

        assert!(id1 != id2);

        let res1 = stream1.recv().await.unwrap();
        let res2 = stream2.recv().await.unwrap();

        assert_eq!(res1, res2);
        assert_res_ok(&res1, &[v.clone()], &[]);

        // 如果 subscriber 取消订阅，则收不到新数据
        let result = b.clone().unsubscribe(lobby.clone(), id1 as _).unwrap();
        assert_eq!(result, id1 as _);

        // publish
        let v: Value = "world".into();
        b.clone().publish(lobby.clone(), Arc::new(v.clone().into()));

        assert!(stream1.recv().await.is_none());
        let res2 = stream2.recv().await.unwrap();
        assert_res_ok(&res2, &[v.clone()], &[]);
    }

    pub async fn get_id(res: &mut Receiver<Arc<CommandResponse>>) -> u32 {
        let id: i64 = res.recv().await.unwrap().as_ref().try_into().unwrap();
        id as u32
    }
}

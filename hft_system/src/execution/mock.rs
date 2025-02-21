use flume::Receiver;
use std::error::Error;
use tracing::info;
use tokio::time::{sleep, Duration};
use crate::core::types::Order;

pub async fn run(rx: Receiver<Order>) -> Result<(), Box<dyn Error>> {
    while let Ok(mut order) = rx.recv_async().await {
        sleep(Duration::from_millis(50)).await; // 模拟延迟
        order.order_id = Some("mock-123".to_string());
        info!("执行卖出策略: {:?}", order);
    }
    Ok(())
}
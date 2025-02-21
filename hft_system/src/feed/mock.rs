use flume::Sender;
use std::error::Error;
use tracing::info;
use tokio::time::{Duration,sleep};
use crate::core::types::MarketData;
use rust_decimal::Decimal;


// 模拟数据生成器
pub async fn run(tx:Sender<MarketData>,interval_ms:u64) -> Result<(),Box<dyn Error>> {
    let mut counter: i64 = 1;
    loop {
        let data = MarketData {
            symbol:"USDT".to_string(),
            price:Decimal::new(95 + counter % 10, 0),
            volume:Decimal::new(10, 0),
            timestamp:std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
        };
        tx.send(data.clone())?;
        info!("获取交易行数据: {:?}", data);  // 记录日志
        counter += 1;    // 增加计数器以改变价格
        sleep(Duration::from_millis(interval_ms)).await;
    }
}
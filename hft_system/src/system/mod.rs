use std::error::Error;
use rust_decimal::Decimal;
use tokio::task;
use tracing::info;
use crate::config::load_config;
use crate::core::types::{MarketData, Order};
use crate::feed::mock as data_feed;
use crate::strategy::{manager, simple::SimpleStrategy, command};
use crate::execution::mock as execution;
use flume::{unbounded, Receiver, Sender};
use crate::strategy::strategy;

// 系统管理器结构体，持有所有通道和任务句柄
pub struct System {
    market_tx: Sender<MarketData>,    // 市场数据发送通道
    market_rx: Receiver<MarketData>,  // 市场数据接收通道
    order_tx: Sender<Order>,          // 订单发送通道
    order_rx: Receiver<Order>,        // 订单接收通道
    command_tx: Sender<command::StrategyCommand>, // 策略调整发送通道
    command_rx: Receiver<command::StrategyCommand>, // 策略调整接收通道
}



impl System {
    pub fn new() -> Self {
        let (market_tx, market_rx) = unbounded();
        let (order_tx, order_rx) = unbounded();
        let (command_tx, command_rx) = unbounded();
        Self {
            market_tx,
            market_rx,
            order_tx,
            order_rx,
            command_tx,
            command_rx,
        }
    }
    pub async fn run_forever(self) -> Result<(),Box<dyn Error>> {
        tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
        let config = load_config("").unwrap();
        info!("Config loaded: {:?}", config);
        let data_handle: task::JoinHandle<()> = task::spawn(async move {
            let _: Result<(), Box<dyn Error>> = data_feed::run(self.market_tx, config.data_feed_interval_ms).await;
        });

        let strategy: Box<dyn strategy::Strategy + Send + 'static> = Box::new(SimpleStrategy::new(config.strategy_threshold)); 

        let strategy_handle: task::JoinHandle<()> = task::spawn(async move {
            let _ = manager::run(self.market_rx, self.order_tx, strategy, self.command_rx).await;
        });
        let execution_handle = task::spawn(async move {
             execution::run(self.order_rx).await.unwrap();
        });
        let control_handle = task::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            info!("发送策略调整命令 更新阈值为 98");
            self.command_tx.send(command::StrategyCommand::UpdateThreshold(Decimal::new(98, 0))).unwrap();

            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            info!("发送策略调整命令 更新阈值为 96");
            self.command_tx.send(command::StrategyCommand::UpdateThreshold(Decimal::new(96, 0))).unwrap();

        });

        // // 使用 try_join! 并处理嵌套 Result
        let _ = tokio::try_join!(data_handle, strategy_handle, execution_handle,control_handle);

        Ok(())
        
    }
}
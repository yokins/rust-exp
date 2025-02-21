use flume::{Sender,Receiver};
use std::error::Error;
use tracing::info;
use crate::core::types::{MarketData, Order};
use crate::strategy::strategy::Strategy;
use crate::strategy::command::StrategyCommand;



pub async fn run(
    data_rx: Receiver<MarketData>,
    order_tx: Sender<Order>,
    mut strategy: Box<dyn Strategy + Send + 'static>,
    command_rx: Receiver<StrategyCommand>,
) -> Result<(),Box<dyn Error>>{
    loop {
        tokio::select! {
            Ok(data) = data_rx.recv_async() => {
                strategy.process(data, order_tx.clone())?;            
            }
            Ok(command) = command_rx.recv_async() => {
                match command {
                    StrategyCommand::UpdateThreshold(threshold) => {
                        strategy.update_threshold(threshold);
                        info!("Strategy threshold updated to {}", threshold);
                    }
                }
            }
        }
    }
}
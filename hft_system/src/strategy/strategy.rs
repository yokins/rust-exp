
use flume::Sender;
use crate::core::types::{MarketData,Order};
use std::error::Error;
use rust_decimal::Decimal;


pub trait Strategy {
    fn update_threshold(&mut self, threshold:Decimal);
    fn name(&self) -> &'static str;
    fn process(&self,data:MarketData,order_tx: Sender<Order>) -> Result<(), Box<dyn Error>>;
}
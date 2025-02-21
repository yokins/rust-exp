use flume::Sender;
use std::error::Error;
use tracing::info;
use crate::core::types::{MarketData, Order,OrderAction};
use rust_decimal::Decimal;

use crate::strategy::strategy::Strategy;

// 简单策略结构体，基于价格阈值生成订单
pub struct SimpleStrategy {
    pub threshold: Decimal,  // 价格阈值，低于此值触发买入
}

impl SimpleStrategy {
    pub fn new(threshold: Decimal) -> Self {
        Self {
            threshold:threshold,
        }
    }
   
}

impl Strategy for SimpleStrategy {

    fn update_threshold(&mut self,threshold:Decimal) {
        self.threshold = threshold;
    }
    fn name(&self) -> &'static str {
        "简单策略"
    }
    fn process(&self,data:MarketData,order_tx: Sender<Order>) -> Result<(), Box<dyn Error>> {
        if self.threshold > data.price {
            self.cell(data,order_tx)
        }else {
            self.buy(data, order_tx)
        }
    }
}


impl SimpleStrategy  {
    fn cell(&self,data:MarketData,order_tx: Sender<Order>) -> Result<(), Box<dyn Error>> {
        let order = Order {
            symbol:data.symbol.clone(),
            action:OrderAction::Cell,
            price:data.price,
            quantity:Decimal::new(1, 0),
            order_id:None,
        };
        order_tx.send(order)?;
        Ok(())
    }
}

impl SimpleStrategy {
    fn buy(&self,data:MarketData,order_tx: Sender<Order>) -> Result<(), Box<dyn Error>> {
        if data.price < self.threshold {
            let order = Order {
                symbol:data.symbol.clone(),
                action:OrderAction::Buy,
                price:data.price,
                quantity:Decimal::new(1, 0),
                order_id:None,
            };
            order_tx.send(order)?;
            info!("命中策略[{}] 购买 {} 本金位 {}",self.name(), data.symbol, data.price);  // 记录触发日志
        }
        Ok(())
    }
}
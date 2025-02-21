// 配置管理模块

use serde::Deserialize;
use std::error::Error;
use config::{Config,File};
use rust_decimal::Decimal;


// 系统配置结构体，定义运行时参数
#[derive(Debug, Deserialize, Clone)]
pub struct SystemConfig {
    pub data_feed_interval_ms: u64,   // 数据采集间隔，单位为毫秒，控制数据刷新频率
    pub strategy_threshold: Decimal,  // 策略触发阈值，例如价格低于此值时买入
    // pub execution_endpoint: String,   // 订单执行的目标端点，例如 "mock" 或真实交易所 URL
}

pub fn load_config(file_path:&str) -> Result<SystemConfig,Box<dyn Error>> {
    let config: Config = Config::builder().add_source(File::with_name(file_path).required(false))
    .set_default("data_feed_interval_ms", 100)?
    .set_default("strategy_threshold", "100.")?
    .set_default("execution_endpoint", "mock")?
    .build()?;
    let settings:SystemConfig = config.try_deserialize()?;
    Ok(settings)
}
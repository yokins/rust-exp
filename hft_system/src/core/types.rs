use rust_decimal::Decimal;



// 市场数据
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct MarketData {
    pub symbol: String,
    pub price: Decimal,
    pub volume: Decimal,
    pub timestamp: u64,
}


// 订单操作方向
#[derive(Debug,Clone)]
pub enum OrderAction {
    Buy,
    Cell,
}


impl std::fmt::Display for OrderAction  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderAction::Buy => write!(f, "BUY"),
            OrderAction::Cell => write!(f, "CELL"),
        }
    }
}


// 订单结构体 交易指令
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Order {
    pub symbol:String,
    pub action: OrderAction,
    pub price:Decimal,
    pub quantity:Decimal,
    pub order_id: Option<String>, // 订单唯一标识符，由交易所返回，初始为 None
}
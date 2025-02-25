use chrono::{TimeZone, Utc};
use serde::Deserialize;
use tabled::settings::{object::Columns, Alignment, Modify, Style};
use tabled::{Table, Tabled};

fn display_option(val: &Option<String>) -> String {
    match val {
        Some(s) => s.clone(),
        None => String::from("N/A"),
    }
}

fn format_timestamp(ts: &str) -> String {
    if let Ok(ts) = ts.parse::<i64>() {
        if let Some(dt) = Utc.timestamp_millis_opt(ts).earliest() {
            return dt.format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }
    String::from("Invalid Timestamp")
}

#[derive(Debug, Deserialize, Tabled)]
struct Ticker {
    #[serde(rename = "instId")]
    #[tabled(rename = "交易对名称")]
    inst_id: String,
    #[serde(rename = "last")]
    #[tabled(rename = "最新成交价")]
    last: String,
    #[serde(rename = "askPx")]
    #[tabled(rename = "卖一价")]
    ask_px: String,
    #[serde(rename = "bidPx")]
    #[tabled(rename = "买一价")]
    bid_px: String,
    #[serde(rename = "vol24h")]
    #[tabled(rename = "24小时成交量", display = "display_option")]
    vol_24h: Option<String>,
    #[serde(rename = "ts")]
    #[tabled(rename = "数据更新时间", display = "format_timestamp")]
    ts: String,
}

#[derive(Debug, Deserialize)]
struct OKXResponse {
    code: String,
    data: Vec<Ticker>,
}

async fn fetch_okx_ticker(symbol: &str) -> anyhow::Result<Ticker> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://www.okx.com/api/v5/market/ticker?instType=SPOT&instId={}",
        symbol
    );

    let response = client
        .get(&url)
        .header("User-agent", "Rust OKX Client/1.0")
        .send()
        .await?;

    let api_response: OKXResponse = response.json().await?;

    if api_response.code != "0" {
        anyhow::bail!("OKX API error: {}", api_response.code);
    }

    api_response
        .data
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No data"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let symbol = "BTC-USDT";
    loop {
        match fetch_okx_ticker(symbol).await {
            Ok(ticker) => {
                let c = vec![ticker];

                let table = Table::new(c)
                    .with(Style::modern())
                    .with(Modify::new(Columns::first()).with(Alignment::center()))
                    .to_string();

                println!("↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓");
                println!("{}", table);
                println!("↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑");
            }
            Err(e) => eprintln!("错误: {}", e),
        }
    }
}

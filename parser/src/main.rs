use serde::Deserialize;
use chrono::Local;
use std::time::Duration;
use std::thread::sleep;

#[derive(Debug, Deserialize)]
struct PriceResponse {
    bitcoin: BtcPrice,
}

#[derive(Debug, Deserialize)]
struct BtcPrice {
    usd: f64,
}

struct PriceHistory {
    prices: Vec<f64>,
    timestamps: Vec<String>,
}

impl PriceHistory {
    fn new() -> Self {
        Self {
            prices: Vec::new(),
            timestamps: Vec::new(),
        }
    }

    fn add(&mut self, price: f64) {
        let now = Local::now().format("%H:%M:%S").to_string();
        self.prices.push(price);
        self.timestamps.push(now);
    }

    fn get_stats(&self) -> (Option<f64>, Option<f64>, Option<f64>) {
        if self.prices.is_empty() {
            return (None, None, None);
        }
        let min = self.prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = self.prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let sum: f64 = self.prices.iter().sum();
        let avg = sum / self.prices.len() as f64;

        (Some(min), Some(max), Some(avg))
    }

    fn show_last(&self, count: usize) {
        let len = self.prices.len();
        let start = if len > count { len - count } else { 0 };

        println!("\n📊 Последние {} записей:", len - start);
        for i in start..len {
            println!("  {} - ${:.2}", self.timestamps[i], self.prices[i]);
        }
    }
}

fn get_btc_price() -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()?;
    let response: PriceResponse = client.get(url).send()?.json()?;
    Ok(response.bitcoin.usd)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Bitcoin Price Tracker Started!");
    println!("📡 Получаем данные от CoinGecko API");
    println!("⏰ Обновление каждые 10 секунд");
    println!("Press Ctrl+C to stop\n");
    println!("{}", "=".repeat(50));

    let mut history = PriceHistory::new();
    loop {
        match get_btc_price() {
            Ok(price) => {
                let now = Local::now().format("%H:%M:%S").to_string();
                history.add(price);
                let (min, max, avg) = history.get_stats();
                println!("[{}] BTC: ${:.2}", now, price);
                if let (Some(min_val), Some(max_val), Some(avg_val)) = (min, max, avg) {
                    println!("   📈 Min: ${:.2}  Max: ${:.2}  Avg: ${:.2}", min_val, max_val, avg_val);
                }
                if history.prices.len() % 5 == 0 && history.prices.len() > 0 {
                    history.show_last(5);
                    println!("{}", "=".repeat(50));
                }
            }
            Err(e) => {
                println!("❌ Ошибка получения цены: {}", e);
            }
        }
        sleep(Duration::from_secs(10));
    }
}

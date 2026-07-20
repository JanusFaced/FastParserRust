use ccxt_exchanges::binance::Binance;
use ccxt_rust::prelude::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Start parsing! ===\n");
    let exchange = setup_exchange().await?;
    
    let symbols = vec!["BTC/USDT", "ETH/USDT", "BNB/USDT"];
    let timeframe = "1m";
    let tail_size = 10;

    for symbol in &symbols {
        println!("✅ symbol =  {} \n", symbol);
       
        let ohlcv_data = exchange
            .fetch_ohlcv(symbol, timeframe, None, None, None)
            .await?;

        print_ohlcv_table(&ohlcv_data, tail_size);
    }

    Ok(())
}

async fn setup_exchange() -> Result<Binance> {
    let exchange = Binance::builder().build()?;
    exchange.load_markets(false).await?;
    println!("✅ Рынки загружены!\n");
    Ok(exchange)
}

fn print_ohlcv_table(ohlcv_data: &[OHLCV], tail_size: usize) {
    println!(
        "{:<20} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "Дата", "Open", "High", "Low", "Close", "Volume"
    );
    println!(
        "{:-<20} {:-<10} {:-<10} {:-<10} {:-<10} {:-<10}",
        "", "", "", "", "", ""
    );
    
    for candle in ohlcv_data.iter().rev().take(tail_size).rev() {
        let date = chrono::DateTime::from_timestamp_millis(candle.timestamp)
            .unwrap()
            .format("%Y-%m-%d %H:%M");
        
        println!(
            "{:>20} {:>10.2} {:>10.2} {:>10.2} {:>10.2} {:>10.2}",
            date, candle.open, candle.high, candle.low, candle.close, candle.volume
        );
    }
    println!();
}
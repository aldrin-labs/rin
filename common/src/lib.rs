use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

// DEX Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub price: f64,
    pub size: f64,
    pub side: OrderSide,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub price: f64,
    pub size: f64,
    pub side: OrderSide,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookData {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub symbol: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub price_decimals: u8,
    pub size_decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

// Mock data generator for testing
pub mod mock {
    use super::*;

    pub fn generate_mock_orderbook() -> OrderBookData {
        let base_price = 94319.4;
        let bids: Vec<Order> = (0..10).map(|i| {
            Order {
                price: base_price - (i as f64 * 0.5),
                size: 0.21551196,
                side: OrderSide::Buy,
            }
        }).collect();

        let asks: Vec<Order> = (0..10).map(|i| {
            Order {
                price: base_price + (i as f64 * 0.5),
                size: 0.00031220,
                side: OrderSide::Sell,
            }
        }).collect();

        OrderBookData { bids, asks }
    }

    pub fn generate_mock_trades() -> Vec<Trade> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        (0..20).map(|i| {
            Trade {
                price: 94319.4 + (i as f64 * 0.1) * if i % 2 == 0 { 1.0 } else { -1.0 },
                size: 0.00031220,
                side: if i % 2 == 0 { OrderSide::Buy } else { OrderSide::Sell },
                timestamp: now - (i * 60) as u64,
            }
        }).collect()
    }

    pub fn generate_mock_market() -> Market {
        Market {
            symbol: "BTC/USD".to_string(),
            base_currency: "BTC".to_string(),
            quote_currency: "USD".to_string(),
            price_decimals: 1,
            size_decimals: 8,
        }
    }

    pub fn generate_mock_candles() -> Vec<Candle> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let base_price = 94319.4;

        (0..24).map(|i| {
            let hour = i as f64;
            Candle {
                timestamp: now - (i * 3600) as u64,
                open: base_price + (hour * 10.0),
                high: base_price + (hour * 10.0) + 5.0,
                low: base_price + (hour * 10.0) - 5.0,
                close: base_price + (hour * 10.0) + 2.0,
                volume: 100.0 + (i as f64 * 10.0),
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_data_generation() {
        let orderbook = mock::generate_mock_orderbook();
        assert!(!orderbook.bids.is_empty());
        assert!(!orderbook.asks.is_empty());

        let trades = mock::generate_mock_trades();
        assert!(!trades.is_empty());


        let market = mock::generate_mock_market();
        assert_eq!(market.symbol, "BTC/USD");

        let candles = mock::generate_mock_candles();
        assert!(!candles.is_empty());
    }
}

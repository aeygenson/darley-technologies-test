use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, PartialEq)]
pub struct InstrumentStats {
    pub symbol: String,
    pub price_change: f64,
    pub price_change_percent: f64,
    pub last_price: f64,
    pub last_qty: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub amount: f64,
    pub bid_price: f64,
    pub ask_price: f64,
    pub open_time: u64,  // Changed from i64 to u64
    pub close_time: u64, // Changed from i64 to u64
    pub first_trade_id: u64,
    pub trade_count: u64,
    pub strike_price: f64,
    pub exercise_price: f64,
}

//O(n) time complexity (linear time complexity) depending on the number of instruments in the input data
pub fn parse_instrument_stats(data: &Value) -> Vec<InstrumentStats> {
    let mut stats = Vec::new();

    if let Some(array) = data.as_array() {
        for item in array {
            if let (
                Some(symbol), Some(price_change), Some(price_change_percent), Some(last_price), Some(last_qty),
                Some(open), Some(high), Some(low), Some(volume), Some(amount), Some(bid_price), Some(ask_price),
                Some(open_time), Some(close_time), Some(first_trade_id), Some(trade_count), Some(strike_price), Some(exercise_price)
            ) = (
                item["symbol"].as_str(),
                item["priceChange"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["priceChangePercent"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["lastPrice"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["lastQty"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["open"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["high"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["low"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["volume"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["amount"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["bidPrice"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["askPrice"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["openTime"].as_u64(),
                item["closeTime"].as_u64(),
                item["firstTradeId"].as_u64(),
                item["tradeCount"].as_u64(),
                item["strikePrice"].as_str().and_then(|s| s.parse::<f64>().ok()),
                item["exercisePrice"].as_str().and_then(|s| s.parse::<f64>().ok())
            ) {
                stats.push(InstrumentStats {
                    symbol: symbol.to_string(),
                    price_change,
                    price_change_percent,
                    last_price,
                    last_qty,
                    open,
                    high,
                    low,
                    volume,
                    amount,
                    bid_price,
                    ask_price,
                    open_time,
                    close_time,
                    first_trade_id,
                    trade_count,
                    strike_price,
                    exercise_price,
                });
            }
        }
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_instrument_stats() {
        let sample_data = json!([
            {
                "symbol": "BTC-200730-9000-C",
                "priceChange": "-16.2038",
                "priceChangePercent": "-0.0162",
                "lastPrice": "1000",
                "lastQty": "1000",
                "open": "1016.2038",
                "high": "1016.2038",
                "low": "0",
                "volume": "5",
                "amount": "1",
                "bidPrice": "999.34",
                "askPrice": "1000.23",
                "openTime": 1592317127349u64,  // Ensure this is u64
                "closeTime": 1592380593516u64, // Ensure this is u64
                "firstTradeId": 1,
                "tradeCount": 5,
                "strikePrice": "9000",
                "exercisePrice": "3000.3356"
            }
        ]);

        let expected = vec![InstrumentStats {
            symbol: "BTC-200730-9000-C".to_string(),
            price_change: -16.2038,
            price_change_percent: -0.0162,
            last_price: 1000.0,
            last_qty: 1000.0,
            open: 1016.2038,
            high: 1016.2038,
            low: 0.0,
            volume: 5.0,
            amount: 1.0,
            bid_price: 999.34,
            ask_price: 1000.23,
            open_time: 1592317127349,
            close_time: 1592380593516,
            first_trade_id: 1,
            trade_count: 5,
            strike_price: 9000.0,
            exercise_price: 3000.3356,
        }];

        let result = parse_instrument_stats(&sample_data);
        assert_eq!(result, expected);
    }
}

use influxdb2_derive::FromDataPoint;

#[derive(FromDataPoint)]
struct StockPrice {
    ticker: String,
    price:  u64,
    time:   chrono::DateTime<chrono::FixedOffset>,
}

impl Default for StockPrice {
    fn default() -> Self {
        Self {
            ticker: "".to_string(),
            price: 0,
            time: chrono::DateTime::parse_from_rfc3339("2020-02-17T22:19:49.747562847Z").unwrap(),
        }
    }
}

fn main() {}


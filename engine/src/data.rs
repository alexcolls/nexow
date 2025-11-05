use crate::types::*;
use rand::Rng;
use chrono::{Utc, Duration};

pub fn generate_synthetic_bars(
    symbol: &str,
    start_price: f64,
    interval_ms: u64,
    n: usize,
    vol: f64,
) -> Vec<Bar> {
    let mut rng = rand::thread_rng();
    let mut price = start_price;
    let mut ts = Utc::now();
    let mut bars = Vec::with_capacity(n);

    for _ in 0..n {
        let ret = rng.gen_range(-vol..vol);
        let open = price;
        price = (price * (1.0 + ret)).max(0.0001);
        let close = price;
        let high = open.max(close) * (1.0 + rng.gen_range(0.0..vol.abs()));
        let low = open.min(close) * (1.0 - rng.gen_range(0.0..vol.abs()));
        let volume = rng.gen_range(1000.0..10000.0);

        bars.push(Bar {
            ts,
            open,
            high,
            low,
            close,
            volume,
            symbol: symbol.to_string(),
        });

        ts += Duration::milliseconds(interval_ms as i64);
    }

    bars
}

pub fn train_test_split(bars: &[Bar], split: f32) -> (&[Bar], &[Bar]) {
    let idx = ((bars.len() as f32) * split).round() as usize;
    let idx = idx.min(bars.len());
    (&bars[..idx], &bars[idx..])
}

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    pub symbol: String,
    pub lot_size: f64,
    pub tick_size: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bar {
    pub ts: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub symbol: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OrderType {
    Market,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub side: Side,
    pub qty: f64,
    pub ty: OrderType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    pub order_id: u64,
    pub price: f64,
    pub qty: f64,
    pub symbol: String,
    pub ts: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub qty: f64,
    pub avg_price: f64,
    pub unrealized_pnl: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Portfolio {
    pub cash: f64,
    pub positions: Vec<Position>,
    pub equity: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub pnl: f64,
    pub max_drawdown: f64,
    pub sharpe: f64,
    pub win_rate: f64,
    pub trades: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Mode {
    Simulate,
    Backtest,
    Forwardtest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EngineConfig {
    pub symbols: Vec<String>,
    pub bar_interval_ms: u64,
    pub length_bars: usize,
    pub rf_trees: usize,
    pub rf_max_depth: usize,
    pub train_split: f32,
    pub mode: Mode,
    pub starting_cash: f64,
}

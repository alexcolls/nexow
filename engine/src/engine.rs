use crate::{data::*, strategy::*, types::*};
use crossbeam_channel::{unbounded, Receiver, Sender};

#[derive(Clone, Debug, serde::Serialize)]
pub enum EngineEvent {
    Bar(Bar),
    Order(Order),
    Trade(Trade),
    Metrics(Metrics),
    Done,
}

pub struct EngineHandle {
    pub tx_ctrl: Sender<EngineControl>,
    pub rx_evt: Receiver<EngineEvent>,
}

pub enum EngineControl {
    Stop,
}

pub struct Engine;

impl Engine {
    pub fn spawn(config: EngineConfig) -> EngineHandle {
        let (tx_evt, rx_evt) = unbounded();
        let (tx_ctrl, rx_ctrl) = unbounded();

        std::thread::spawn(move || {
            let mut strategy = RfStrategy::new(config.rf_trees, config.rf_max_depth);

            // Synthetic data for first symbol only for MVP
            let symbol = config.symbols.get(0).cloned().unwrap_or_else(|| "SIM".into());
            let bars = generate_synthetic_bars(&symbol, 100.0, config.bar_interval_ms, config.length_bars, 0.01);
            let (train, test) = train_test_split(&bars, config.train_split);
            let _ = strategy.train(train);

            let mut cash = config.starting_cash;
            let mut qty: f64 = 0.0;
            let mut entry_price = 0.0;
            let mut wins = 0usize;
            let mut losses = 0usize;
            let mut pnl = 0.0f64;
            let mut peak_equity = cash;

            for b in test.iter().cloned() {
                if let Ok(ctrl) = rx_ctrl.try_recv() {
                    match ctrl {
                        EngineControl::Stop => break,
                    }
                }

                let long = strategy.decide(&b);

                if long && qty == 0.0 {
                    // enter long with 10% cash
                    let spend = cash * 0.10;
                    qty = spend / b.close;
                    entry_price = b.close;
                    cash -= spend;
                    let _ = tx_evt.send(EngineEvent::Order(Order {
                        id: 0,
                        symbol: symbol.clone(),
                        side: Side::Buy,
                        qty,
                        ty: OrderType::Market,
                    }));
                } else if !long && qty > 0.0 {
                    // exit position
                    let proceeds = qty * b.close;
                    let trade_pnl = proceeds - (qty * entry_price);
                    pnl += trade_pnl;
                    if trade_pnl >= 0.0 {
                        wins += 1;
                    } else {
                        losses += 1;
                    }
                    cash += proceeds;
                    qty = 0.0;
                    let _ = tx_evt.send(EngineEvent::Order(Order {
                        id: 0,
                        symbol: symbol.clone(),
                        side: Side::Sell,
                        qty: 0.0,
                        ty: OrderType::Market,
                    }));
                }

                let equity = cash + qty * b.close;
                if equity > peak_equity {
                    peak_equity = equity;
                }
                let dd = (peak_equity - equity) / peak_equity.max(1.0);

                let m = Metrics {
                    pnl,
                    max_drawdown: dd,
                    sharpe: 0.0, // compute later with returns vector
                    win_rate: if wins + losses > 0 {
                        wins as f64 / (wins + losses) as f64
                    } else {
                        0.0
                    },
                    trades: wins + losses,
                };

                let _ = tx_evt.send(EngineEvent::Bar(b));
                let _ = tx_evt.send(EngineEvent::Metrics(m));
            }

            let _ = tx_evt.send(EngineEvent::Done);
        });

        EngineHandle { tx_ctrl, rx_evt }
    }
}

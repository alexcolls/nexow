use axum::{
    extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::state::AppState;
use nexow_engine::{Engine, EngineConfig, EngineEvent, Mode};

#[derive(Deserialize)]
pub struct StartSimReq {
    pub symbols: Vec<String>,
    pub bar_interval_ms: u64,
    pub length_bars: usize,
    pub rf_trees: usize,
    pub rf_max_depth: usize,
    pub train_split: f32,
    pub mode: String,
    pub starting_cash: f64,
}

#[derive(Serialize)]
pub struct StartSimRes {
    status: String,
}

pub async fn start_sim(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StartSimReq>,
) -> Json<StartSimRes> {
    let mode = match req.mode.as_str() {
        "backtest" => Mode::Backtest,
        "forwardtest" => Mode::Forwardtest,
        _ => Mode::Simulate,
    };

    let cfg = EngineConfig {
        symbols: req.symbols,
        bar_interval_ms: req.bar_interval_ms,
        length_bars: req.length_bars,
        rf_trees: req.rf_trees,
        rf_max_depth: req.rf_max_depth,
        train_split: req.train_split,
        mode,
        starting_cash: req.starting_cash,
    };

    let handle = Engine::spawn(cfg);

    {
        let mut eng = state.engine.lock().unwrap();
        *eng = Some(handle);
    }

    Json(StartSimRes {
        status: "started".into(),
    })
}

pub async fn sim_status(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "running": state.engine.lock().unwrap().is_some()
    }))
}

pub async fn ws_stream(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(mut socket: WebSocket, state: Arc<AppState>) {
    let rx = {
        let eng = state.engine.lock().unwrap();
        if let Some(h) = eng.as_ref() {
            Some(h.rx_evt.clone())
        } else {
            None
        }
    };

    if rx.is_none() {
        return;
    }

    let rx = rx.unwrap();

    tokio::task::spawn_blocking(move || {
        while let Ok(evt) = rx.recv() {
            if let Ok(s) = serde_json::to_string(&evt) {
                // best-effort; ignore send errors on closed socket
                let _ = futures::executor::block_on(async { socket.send(Message::Text(s)).await });
            }
        }
    });
}

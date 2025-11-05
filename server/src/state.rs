use nexow_engine::EngineHandle;
use sqlx::PgPool;
use std::sync::Mutex;

pub struct AppState {
    pub engine: Mutex<Option<EngineHandle>>,
    pub pool_app: PgPool,
    pub pool_ts: PgPool,
}

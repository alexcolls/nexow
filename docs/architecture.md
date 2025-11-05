# Nexow Architecture

## Overview

Nexow is a modular, event-driven trading platform built with Rust and Nuxt. The architecture is designed for low-latency execution, real-time data streaming, and horizontal scalability.

## System Components

### 1. Engine (Rust Library)

**Purpose**: Core trading engine with strategy execution, synthetic data generation, and event streaming.

**Key Modules**:
- `types.rs`: Domain types (Bar, Order, Trade, Position, Metrics, etc.)
- `data.rs`: Synthetic data generation for simulation
- `strategy.rs`: Trading strategies (currently Random Forest classifier)
- `engine.rs`: Event loop, execution logic, and control flow

**Architecture Pattern**: Event-driven with channel-based communication

**Key Features**:
- Thread-based execution for CPU-intensive operations
- Crossbeam channels for lock-free communication
- Synthetic random-walk bar generation with configurable volatility
- ML-based strategy using smartcore Random Forest classifier
- Real-time metrics calculation (PnL, drawdown, Sharpe, win rate)

### 2. Server (Axum Binary)

**Purpose**: HTTP/WebSocket API server integrating the engine with databases.

**Key Modules**:
- `main.rs`: Server setup, routing, and lifecycle management
- `routes.rs`: API endpoints and WebSocket handlers
- `state.rs`: Application state (engine handle, DB pools)
- `db.rs`: Database connection management

**API Endpoints**:
- `GET /health` - Health check
- `GET /api/assets` - List available assets (placeholder)
- `POST /api/sim/start` - Start simulation/backtest
- `GET /api/sim/status` - Check engine status
- `GET /ws/stream` - WebSocket for real-time event streaming

**Database Connections**:
- **PostgreSQL (App)**: Application data (runs, assets, users)
- **TimescaleDB**: Time-series data (bars, trades, metrics)
- **Qdrant**: Vector database (reserved for ML features)

### 3. CLI (Rust Binary)

**Purpose**: Orchestrator for managing the entire platform lifecycle.

**Commands**:
- `nexow start` - Start all services (databases, server, station)
- `nexow stop` - Stop all services
- `nexow db up` - Start database containers
- `nexow db down` - Stop database containers
- `nexow db status` - Check database health
- `nexow check` - Verify prerequisites (TODO)

**Responsibilities**:
- Docker Compose management
- Process spawning and monitoring
- Health checking and readiness probes
- PID file management for graceful shutdown

### 4. Station (Nuxt 3 SPA)

**Purpose**: Web-based user interface for trading, analysis, and monitoring.

**Key Features**:
- SSR disabled (pure SPA for performance)
- Pinia state management
- WebSocket integration for real-time updates
- Tailwind CSS for styling
- Absolute imports via path aliases

**Pages** (TODO):
- `/dashboard` - Real-time metrics and monitoring
- `/assets` - Available assets and symbols
- `/simulate` - Start simulations and backtests
- `/strategies` - Strategy management
- `/portfolio` - Portfolio overview

## Data Flow

### Simulation/Backtest Flow

```
User (Station) 
  → POST /api/sim/start 
  → Server spawns Engine thread
  → Engine generates synthetic bars
  → Engine trains RF strategy
  → Engine executes strategy on test data
  → Engine emits events (Bar, Order, Metrics)
  → Server streams events via WebSocket
  → Station receives and displays updates
```

### Event Streaming

```
Engine Thread
  ↓ (Crossbeam channel)
EngineHandle.rx_evt
  ↓ (WebSocket upgrade)
Server WebSocket handler
  ↓ (JSON serialization)
Station WebSocket client
  ↓ (Pinia store)
Vue Components
```

## Process Boundaries

### Local Development

```
┌─────────────────────────────────────────┐
│  Docker Compose                          │
│  ┌──────────┐ ┌──────────┐ ┌────────┐  │
│  │Postgres  │ │TimescaleDB│ │Qdrant  │  │
│  │(app)     │ │(ts data)  │ │(vector)│  │
│  └──────────┘ └──────────┘ └────────┘  │
└─────────────────────────────────────────┘
            ↕ (SQL/gRPC)
┌─────────────────────────────────────────┐
│  Nexow Server (Axum)                     │
│  ┌────────────────────────────────────┐ │
│  │  Engine Thread                     │ │
│  │  (Random Forest Strategy)          │ │
│  └────────────────────────────────────┘ │
└─────────────────────────────────────────┘
            ↕ (HTTP/WS)
┌─────────────────────────────────────────┐
│  Station (Nuxt 3 SPA)                    │
│  - Vue 3 Composition API                 │
│  - Pinia Stores                          │
│  - WebSocket Client                      │
└─────────────────────────────────────────┘
```

### Future: Distributed Deployment

```
┌──────────┐      ┌──────────┐      ┌──────────┐
│  Engine  │      │  Engine  │      │  Engine  │
│  Worker  │      │  Worker  │      │  Worker  │
└──────────┘      └──────────┘      └──────────┘
       ↕                ↕                 ↕
  ┌────────────────────────────────────────────┐
  │         Message Queue (Redis/NATS)          │
  └────────────────────────────────────────────┘
       ↕                ↕                 ↕
┌──────────┐      ┌──────────┐      ┌──────────┐
│  API     │      │  API     │      │  API     │
│  Server  │      │  Server  │      │  Server  │
└──────────┘      └──────────┘      └──────────┘
```

## Database Schema

### PostgreSQL (App Database)

```sql
-- Simulation/backtest runs
CREATE TABLE runs (
  id UUID PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  mode TEXT NOT NULL,              -- 'simulate' | 'backtest' | 'forwardtest'
  symbols TEXT[] NOT NULL,
  config JSONB,                     -- Engine configuration
  status TEXT NOT NULL              -- 'running' | 'completed' | 'failed'
);

-- Trading assets
CREATE TABLE assets (
  symbol TEXT PRIMARY KEY,
  lot_size DOUBLE PRECISION NOT NULL DEFAULT 1.0,
  tick_size DOUBLE PRECISION NOT NULL DEFAULT 0.01,
  asset_type TEXT,                  -- 'crypto' | 'stock' | 'forex' | etc.
  metadata JSONB
);
```

### TimescaleDB (Time-Series Data)

```sql
-- OHLCV bars (hypertable)
CREATE TABLE bars (
  ts TIMESTAMPTZ NOT NULL,
  symbol TEXT NOT NULL,
  open DOUBLE PRECISION,
  high DOUBLE PRECISION,
  low DOUBLE PRECISION,
  close DOUBLE PRECISION,
  volume DOUBLE PRECISION,
  PRIMARY KEY (ts, symbol)
);

SELECT create_hypertable('bars', 'ts', if_not_exists => TRUE);

-- Trade history (hypertable)
CREATE TABLE trades (
  ts TIMESTAMPTZ NOT NULL,
  run_id UUID NOT NULL,
  symbol TEXT NOT NULL,
  side TEXT NOT NULL,              -- 'buy' | 'sell'
  price DOUBLE PRECISION NOT NULL,
  quantity DOUBLE PRECISION NOT NULL,
  PRIMARY KEY (ts, run_id, symbol)
);

SELECT create_hypertable('trades', 'ts', if_not_exists => TRUE);
```

### Qdrant (Vector Database)

Reserved for future ML features:
- Strategy embeddings
- Pattern recognition
- Similar trade search
- Market regime classification

## Security Considerations

### Current (Development)

- No authentication (local development only)
- CORS allows all origins
- Database credentials in `.env`
- WebSocket connections unauthenticated

### Future (Production)

- [ ] JWT-based authentication
- [ ] CORS restricted to known origins
- [ ] Secrets managed via vault (HashiCorp Vault, AWS Secrets Manager)
- [ ] WebSocket authentication with tokens
- [ ] Rate limiting on API endpoints
- [ ] TLS/SSL for all connections
- [ ] Database connection pooling with limits
- [ ] Input validation and sanitization

## Performance Characteristics

### Engine

- **Latency**: ~1-10ms per bar (simulation)
- **Throughput**: ~100-1000 bars/second
- **Memory**: ~10-50MB per simulation
- **Concurrency**: Single-threaded per simulation (multiple sims = multiple threads)

### Server

- **Latency**: <1ms for health checks
- **Throughput**: ~1000 req/s (HTTP), unlimited (WebSocket)
- **Memory**: ~50-100MB base + engine overhead
- **Connections**: Pooled (5 per database)

### Station

- **Initial Load**: ~500KB-1MB (SPA bundle)
- **Runtime Memory**: ~50-100MB
- **WebSocket Overhead**: Minimal (<1KB/event)

## Scalability

### Vertical Scaling

- Increase server resources (CPU, RAM)
- Optimize database queries and indices
- Tune connection pool sizes

### Horizontal Scaling

- Deploy multiple API servers behind load balancer
- Distribute engine workers across machines
- Use message queue for engine coordination
- Shard databases by symbol or time range

## Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Engine | Rust (std, smartcore) | Low-latency execution |
| Server | Axum, Tokio, SQLx | Async HTTP/WS API |
| CLI | Clap, Tokio | Process orchestration |
| Station | Nuxt 3, Vue 3, Pinia | User interface |
| Databases | PostgreSQL, TimescaleDB, Qdrant | Data persistence |
| Infrastructure | Docker, Docker Compose | Local development |

## Future Enhancements

### Python SDK

- PyO3-based Python bindings
- Poetry for dependency management
- Strategy development in Python
- Backtesting API wrapper
- Jupyter notebook integration

### Broker Integration

- Interactive Brokers (TWS API)
- Binance (REST + WebSocket)
- Coinbase, Kraken, FTX
- Alpaca, TD Ameritrade
- Generic FIX protocol adapter

### Smart Contract Integration

- Solana program integration
- Ethereum smart contracts (Solidity)
- Bitcoin Script (limited)
- Cross-chain bridges
- On-chain settlement and liquidation

## Development Workflow

1. **Local Development**: Docker Compose + cargo + yarn
2. **Testing**: `cargo test`, Nuxt test utils
3. **Linting**: rustfmt, clippy, ESLint
4. **CI/CD**: GitHub Actions (build, test, lint)
5. **Deployment**: Docker images, Kubernetes (future)

## Monitoring & Observability

### Current

- Server logs via `tracing` crate
- Health check endpoint
- Manual database queries

### Future

- [ ] Prometheus metrics export
- [ ] Grafana dashboards
- [ ] Distributed tracing (Jaeger/Zipkin)
- [ ] Error tracking (Sentry)
- [ ] Performance profiling

## References

- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Axum Documentation](https://docs.rs/axum)
- [Nuxt 3 Documentation](https://nuxt.com/docs)
- [TimescaleDB Best Practices](https://docs.timescale.com/timescaledb/latest/how-to-guides/)

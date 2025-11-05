# Nexow

**Open-source, low-latency trading platform for algorithmic trading**

Nexow is a professional trading framework built with Rust and Nuxt, designed for building, testing, and deploying algorithmic trading strategies across multiple asset classes.

## Features

- 🦀 **Rust Core Engine** - Ultra-low-latency execution with Random Forest ML strategies
- 🛰️ **Axum HTTP/WebSocket Server** - Real-time event streaming and API
- 🎨 **Nuxt 3 SPA Station** - Beautiful web interface for trading and analysis
- 🐳 **Docker Infrastructure** - PostgreSQL, TimescaleDB, and Qdrant
- 📊 **Simulated Trading Book** - Test strategies with synthetic data
- 🤖 **ML Integration** - Built-in Random Forest classifier with smartcore
- 📈 **Performance Metrics** - PnL, drawdown, Sharpe ratio, win rate tracking

## License

MIT License - See [LICENSE](LICENSE) for details

## Architecture

```
nexow/
├── engine/     # Rust core trading engine (library)
├── server/     # Axum API + WebSocket backend
├── cli/        # nexow orchestrator CLI
├── station/    # Nuxt 3 SPA frontend
├── docker/     # Database initialization scripts
└── docs/       # Documentation
```

### Components

- **Engine**: Low-latency trading engine with synthetic data generation, ML strategies, and event-driven architecture
- **Server**: Axum-based API server integrating the engine with PostgreSQL (app data) and TimescaleDB (historical bars)
- **Station**: Nuxt 3 SPA providing UI for simulation, backtesting, and real-time monitoring
- **CLI**: Orchestrator managing Docker services, server, and station processes

## Quickstart

### Prerequisites

See [docs/prerequisites.md](docs/prerequisites.md) for detailed setup instructions.

Required:
- Rust toolchain (rustup, cargo, rustc)
- Node.js 18+ and Yarn (via corepack)
- Docker and Docker Compose
- Git

### Installation

1. **Clone and setup environment**:
```bash
git clone <repository-url>
cd nexow
cp .env.sample .env
# Edit .env with your configuration
```

2. **Install Rust dependencies** (builds workspace):
```bash
cargo build --workspace
```

3. **Install Node dependencies** (for station):
```bash
cd station
corepack enable
yarn install
cd ..
```

4. **Start databases**:
```bash
cargo run -p nexow -- db up
# Or: docker compose up -d
```

5. **Run server** (in one terminal):
```bash
cargo run -p nexow-server
```

6. **Run station** (in another terminal):
```bash
cd station
yarn dev
```

7. **Access the platform**:
- Station UI: http://127.0.0.1:3000
- API Server: http://127.0.0.1:8080
- Health check: http://127.0.0.1:8080/health

### Quick Test

```bash
# Start a simulation via API
curl -X POST http://127.0.0.1:8080/api/sim/start \
  -H "Content-Type: application/json" \
  -d '{
    "symbols": ["BTC-USD"],
    "bar_interval_ms": 1000,
    "length_bars": 2000,
    "rf_trees": 100,
    "rf_max_depth": 8,
    "train_split": 0.7,
    "mode": "simulate",
    "starting_cash": 100000
  }'

# Connect to WebSocket for live updates
# ws://127.0.0.1:8080/ws/stream
```

## Configuration

All configuration is managed through environment variables. Copy `.env.sample` to `.env` and customize:

- **Logging**: `RUST_LOG` (info, debug, trace)
- **Server**: Host, port, and URL configuration
- **Databases**: PostgreSQL, TimescaleDB, Qdrant connection details
- **Engine**: Default simulation parameters

See `.env.sample` for complete configuration options.

## Development

### Project Structure

```
engine/src/
├── types.rs      # Core domain types (Bar, Order, Position, Metrics)
├── data.rs       # Synthetic data generation
├── strategy.rs   # Trading strategies (RF classifier)
└── engine.rs     # Event loop and execution engine

server/src/
├── main.rs       # Server entry point and routing
├── routes.rs     # API and WebSocket handlers
├── state.rs      # Application state
└── db.rs         # Database connections

cli/src/
└── main.rs       # CLI orchestrator (start/stop/db/check)

station/
├── pages/        # Vue pages (dashboard, assets, simulate)
├── stores/       # Pinia stores
└── nuxt.config.ts
```

### Building

```bash
# Build all Rust crates
cargo build --workspace

# Build in release mode
cargo build --workspace --release

# Run tests
cargo test --workspace

# Format code
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings
```

### Database Management

```bash
# Start databases
cargo run -p nexow -- db up

# Stop databases
cargo run -p nexow -- db down

# Check status
cargo run -p nexow -- db status
```

## Roadmap

See [docs/TODO.md](docs/TODO.md) for the complete roadmap.

### Phase 1: MVP (Current)
- [x] Rust engine with synthetic data
- [x] Random Forest ML strategy
- [x] Axum server with WebSocket streaming
- [x] Docker infrastructure
- [ ] Nuxt Station SPA
- [ ] CLI orchestrator

### Phase 2: Real Data
- [ ] Broker/exchange connectivity (Interactive Brokers, Binance, etc.)
- [ ] Live data feeds
- [ ] Historical data ingestion to TimescaleDB
- [ ] Real-time market data streaming

### Phase 3: Advanced Features
- [ ] Python SDK for strategy development
- [ ] Multiple strategy types and backtesting framework
- [ ] Walk-forward analysis and optimization
- [ ] Advanced risk management
- [ ] Portfolio management
- [ ] Multi-user authentication

### Phase 4: DeFi Integration
- [ ] Smart contract integration (Solana, Ethereum, Bitcoin)
- [ ] On-chain settlement and liquidation
- [ ] Decentralized strategy marketplace

## Performance

The Rust engine is designed for low-latency trading:
- Event-driven architecture with channel-based communication
- Zero-copy data structures where possible
- Async I/O with Tokio runtime
- Optimized for high-frequency operations

## Contributing

Contributions are welcome! Please read our contributing guidelines (TODO) and code of conduct (TODO).

## Support

- Documentation: [docs/](docs/)
- Issues: GitHub Issues
- Discussions: GitHub Discussions

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Nuxt 3](https://nuxt.com/) - Vue.js framework
- [smartcore](https://github.com/smartcorelib/smartcore) - Machine learning library
- [sqlx](https://github.com/launchbadge/sqlx) - SQL toolkit
- [TimescaleDB](https://www.timescale.com/) - Time-series database
- [Qdrant](https://qdrant.tech/) - Vector database

---

**Nexow** - Building the future of algorithmic trading, open source.

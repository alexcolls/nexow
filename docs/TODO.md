# Nexow TODO & Roadmap

This document tracks planned features, enhancements, and known issues.

## Current Status

### ✅ Completed (Phase 1 - MVP Foundation)

- [x] MIT License and project structure
- [x] Rust Cargo workspace (engine, server, CLI)
- [x] Core trading engine with synthetic data
- [x] Random Forest ML strategy (smartcore)
- [x] Event-driven architecture with channels
- [x] Performance metrics (PnL, drawdown, Sharpe, win rate)
- [x] Axum HTTP/WebSocket server
- [x] Real-time event streaming via WebSocket
- [x] PostgreSQL integration
- [x] TimescaleDB integration
- [x] Qdrant integration (ready)
- [x] Docker Compose infrastructure
- [x] Environment variable configuration
- [x] Basic documentation (README, architecture)

### 🚧 In Progress (Phase 1 - MVP Completion)

- [ ] Nuxt 3 Station (SPA frontend)
  - [ ] Initialize Nuxt project with SSR disabled
  - [ ] Configure Pinia state management
  - [ ] Add Tailwind CSS styling
  - [ ] Create dashboard page with real-time metrics
  - [ ] Create simulate page with form
  - [ ] WebSocket integration for live updates
  - [ ] Chart library integration (Chart.js / Apache ECharts)
  
- [ ] CLI Orchestrator
  - [ ] Implement `nexow start` (spawn server + station + wait for health)
  - [ ] Implement `nexow stop` (graceful shutdown with PID files)
  - [ ] Implement `nexow check` (verify prerequisites)
  - [ ] Health check polling for readiness
  - [ ] Better error handling and user feedback

- [ ] Quality & CI
  - [ ] rustfmt configuration
  - [ ] clippy configuration
  - [ ] GitHub Actions CI workflow
  - [ ] Automated testing on PR
  - [ ] Code coverage reporting

## Phase 2: Real Data & Connectivity

### Broker/Exchange Integration

- [ ] **Interactive Brokers (IBKR)**
  - [ ] TWS API client
  - [ ] Real-time market data subscription
  - [ ] Order placement and management
  - [ ] Account information retrieval
  - [ ] Historical data download

- [ ] **Binance**
  - [ ] REST API client
  - [ ] WebSocket market data streams
  - [ ] Order execution
  - [ ] Account balances and positions
  - [ ] Futures and spot trading support

- [ ] **Generic Broker Adapter**
  - [ ] Abstract broker trait
  - [ ] FIX protocol support
  - [ ] Unified order management
  - [ ] Multi-broker routing

### Data Management

- [ ] **Historical Data Ingestion**
  - [ ] CSV import to TimescaleDB
  - [ ] Direct broker/exchange downloads
  - [ ] Data validation and cleaning
  - [ ] Gap detection and filling

- [ ] **Real-Time Data Streaming**
  - [ ] Live market data ingestion
  - [ ] Multiple symbol subscriptions
  - [ ] Data normalization across sources
  - [ ] Buffering and backpressure handling

- [ ] **Data API**
  - [ ] REST endpoints for historical queries
  - [ ] WebSocket for real-time data
  - [ ] Data export (CSV, JSON, Parquet)
  - [ ] Aggregation and resampling

## Phase 3: Advanced Trading Features

### Strategy Development

- [ ] **Python SDK**
  - [ ] PyO3 bindings for Rust engine
  - [ ] Poetry-based package
  - [ ] Strategy base class
  - [ ] Indicators library (TA-Lib integration)
  - [ ] Jupyter notebook examples
  - [ ] Documentation and tutorials

- [ ] **Strategy Types**
  - [ ] Trend following
  - [ ] Mean reversion
  - [ ] Arbitrage (stat arb, pairs trading)
  - [ ] Market making
  - [ ] Options strategies
  - [ ] Custom strategy framework

- [ ] **Backtesting Framework**
  - [ ] Walk-forward analysis (WFA)
  - [ ] Monte Carlo simulation
  - [ ] Parameter optimization
  - [ ] Out-of-sample testing
  - [ ] Strategy comparison
  - [ ] Performance attribution

### Risk Management

- [ ] **Position Sizing**
  - [ ] Fixed fractional
  - [ ] Kelly criterion
  - [ ] Risk parity
  - [ ] Volatility-based

- [ ] **Risk Controls**
  - [ ] Max position size limits
  - [ ] Max drawdown stops
  - [ ] Daily loss limits
  - [ ] Exposure limits by asset/sector
  - [ ] VaR (Value at Risk) calculation

- [ ] **Portfolio Management**
  - [ ] Multi-strategy allocation
  - [ ] Correlation analysis
  - [ ] Portfolio rebalancing
  - [ ] Performance tracking
  - [ ] Attribution analysis

### Monitoring & Analysis

- [ ] **Real-Time Dashboards**
  - [ ] Live P&L tracking
  - [ ] Position monitoring
  - [ ] Order book visualization
  - [ ] Strategy performance metrics
  - [ ] System health monitoring

- [ ] **Analytics**
  - [ ] Trade analysis
  - [ ] Slippage analysis
  - [ ] Fill quality metrics
  - [ ] Market impact analysis
  - [ ] Strategy diagnostics

- [ ] **Alerting**
  - [ ] Email notifications
  - [ ] Telegram/Discord webhooks
  - [ ] SMS alerts (Twilio)
  - [ ] Custom alert rules

## Phase 4: DeFi Integration

### Blockchain Integration

- [ ] **Solana**
  - [ ] Program (smart contract) development
  - [ ] On-chain order book
  - [ ] Token swaps via Jupiter/Serum
  - [ ] Lending protocol integration
  - [ ] Cross-program invocation (CPI)

- [ ] **Ethereum**
  - [ ] Smart contract development (Solidity)
  - [ ] Uniswap/SushiSwap integration
  - [ ] Aave/Compound lending
  - [ ] Flash loans
  - [ ] Gas optimization

- [ ] **Bitcoin**
  - [ ] Lightning Network integration
  - [ ] Multisig wallets
  - [ ] Script-based contracts (limited)
  - [ ] Ordinals/Inscriptions (experimental)

### DeFi Features

- [ ] **Decentralized Settlement**
  - [ ] Atomic swaps
  - [ ] Escrow contracts
  - [ ] Trustless liquidation
  - [ ] Cross-chain bridges

- [ ] **Strategy Marketplace**
  - [ ] Strategy publishing
  - [ ] Performance verification
  - [ ] Subscription model
  - [ ] Revenue sharing (smart contracts)

- [ ] **Tokenization**
  - [ ] Strategy performance tokens
  - [ ] Governance tokens
  - [ ] Liquidity provision rewards

## Phase 5: Enterprise Features

### Multi-User Support

- [ ] **Authentication & Authorization**
  - [ ] JWT-based auth
  - [ ] OAuth2/OIDC integration
  - [ ] Role-based access control (RBAC)
  - [ ] API key management

- [ ] **User Management**
  - [ ] User registration and profiles
  - [ ] Team/organization support
  - [ ] Permission management
  - [ ] Audit logs

### Deployment & Scaling

- [ ] **Cloud Deployment**
  - [ ] Kubernetes manifests
  - [ ] Helm charts
  - [ ] AWS/GCP/Azure support
  - [ ] Infrastructure as Code (Terraform)

- [ ] **Horizontal Scaling**
  - [ ] Load balancer configuration
  - [ ] Engine worker pools
  - [ ] Message queue integration (NATS/Redis)
  - [ ] Database sharding

- [ ] **High Availability**
  - [ ] Failover mechanisms
  - [ ] Data replication
  - [ ] Backup and recovery
  - [ ] Disaster recovery plan

### Compliance & Reporting

- [ ] **Regulatory Compliance**
  - [ ] Trade reporting
  - [ ] Position limits enforcement
  - [ ] Best execution monitoring
  - [ ] MiFID II compliance (EU)
  - [ ] SEC compliance (US)

- [ ] **Reporting**
  - [ ] Performance reports (PDF/Excel)
  - [ ] Tax reporting (1099, capital gains)
  - [ ] Monthly/quarterly statements
  - [ ] Custom report builder

## Technical Debt & Improvements

### Code Quality

- [ ] Add comprehensive unit tests (target: >80% coverage)
- [ ] Add integration tests for API endpoints
- [ ] Add E2E tests for critical flows
- [ ] Improve error handling and recovery
- [ ] Add more detailed logging and tracing
- [ ] Performance benchmarking suite

### Documentation

- [ ] API documentation (OpenAPI/Swagger)
- [ ] User guide
  - [ ] Getting started tutorial
- [ ] Strategy development guide
- [ ] Deployment guide
- [ ] Troubleshooting guide
- [ ] Contributing guidelines
- [ ] Code of conduct

### Refactoring

- [ ] Extract broker abstraction layer
- [ ] Improve WebSocket connection handling
- [ ] Better configuration management
- [ ] Database migration system (SQLx migrate)
- [ ] Improve CLI user experience
- [ ] Modularize Station components

## Known Issues

- [ ] WebSocket connection not gracefully handled on client disconnect
- [ ] No rate limiting on API endpoints
- [ ] Server panics on database connection failure
- [ ] CLI stub implementation needs completion
- [ ] Station not yet implemented
- [ ] No authentication or security measures
- [ ] CORS allows all origins (insecure)
- [ ] No input validation on API requests
- [ ] Engine metrics don't calculate Sharpe ratio correctly
- [ ] No graceful shutdown handling

## Community & Ecosystem

- [ ] Create Discord community
- [ ] Set up GitHub Discussions
- [ ] Create YouTube tutorial series
- [ ] Write blog posts on architecture
- [ ] Participate in trading/quant forums
- [ ] Create example strategies repository
- [ ] Build strategy template generator
- [ ] Developer onboarding program

## Research & Exploration

- [ ] Reinforcement learning for strategy optimization
- [ ] Transformer models for price prediction
- [ ] Graph neural networks for market structure
- [ ] Zero-knowledge proofs for strategy privacy
- [ ] Homomorphic encryption for secure computation
- [ ] Quantum-resistant cryptography
- [ ] Alternative consensus mechanisms

---

**Last Updated**: 2025-01-05

**Maintainers**: Nexow Core Team

**Contributing**: See [CONTRIBUTING.md](../CONTRIBUTING.md) (TODO)

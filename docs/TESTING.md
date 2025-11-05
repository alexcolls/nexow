# Testing Guide

This document describes how to test the Nexow platform end-to-end.

## Prerequisites Check

Before testing, verify all required tools are installed:

```bash
cargo run -p nexow -- check
```

Expected output:
```
🔍 Checking prerequisites...

✅ docker: Docker version 24.x.x
✅ docker: Docker Compose version v2.x.x
✅ cargo: cargo 1.x.x
✅ rustc: rustc 1.x.x
✅ node: v18.x.x or higher
✅ yarn: 1.22.x

✅ All prerequisites satisfied
```

## Database Services

### Start Databases

```bash
cargo run -p nexow -- db up
```

This starts:
- PostgreSQL (app data) on port 5432
- TimescaleDB (time-series data) on port 5433
- Qdrant (vector storage) on port 6333

### Check Database Status

```bash
cargo run -p nexow -- db status
```

Expected output shows 3 healthy containers:
```
📊 Database service status:
NAMES                STATUS                  PORTS
nexow_postgres_app   Up X minutes (healthy)  0.0.0.0:5432->5432/tcp
nexow_timescaledb    Up X minutes (healthy)  0.0.0.0:5433->5432/tcp
nexow_qdrant         Up X minutes (healthy)  0.0.0.0:6333->6333/tcp
```

### Stop Databases

```bash
cargo run -p nexow -- db down
```

## Full Platform

### Start Platform

Start both server and station:

```bash
cargo run -p nexow -- start
```

The CLI will:
1. Start Docker services (if not running)
2. Wait for database health checks (up to 30s each)
3. Start Nexow server in background
4. Start Nuxt station in background
5. Wait for HTTP health checks
6. Display access URLs

Expected output:
```
🚀 Starting Nexow platform...
📦 Starting database services...
⏳ Waiting for databases to be healthy...
✅ nexow_postgres_app is healthy
✅ nexow_timescaledb is healthy
✅ nexow_qdrant is healthy
🛰️  Starting Nexow server...
✅ Server healthy at http://127.0.0.1:8080/health
🎨 Starting Nexow station...
✅ Station ready at http://127.0.0.1:3000

🎉 Nexow is running!
   Server:  http://127.0.0.1:8080
   Station: http://127.0.0.1:3000

💡 Use 'nexow stop' to shut down
```

### Skip Health Checks (Faster)

If you want to skip waiting for health checks:

```bash
cargo run -p nexow -- start --no-wait
```

### Stop Platform

Stop server and station only (databases keep running):

```bash
cargo run -p nexow -- stop
```

Stop everything including databases:

```bash
cargo run -p nexow -- stop --with-db
```

## Manual Testing

### 1. Health Check

Verify server is running:

```bash
curl http://127.0.0.1:8080/health
```

Expected: `ok`

### 2. Assets API

Check available assets:

```bash
curl http://127.0.0.1:8080/api/assets
```

Expected: `[]` (empty until exchange connectivity is added)

### 3. Simulation Status

Check if engine is running:

```bash
curl http://127.0.0.1:8080/api/sim/status
```

Expected: `{"running":false}` (or `true` if simulation is active)

### 4. Start Simulation via API

```bash
curl -X POST http://127.0.0.1:8080/api/sim/start \
  -H "Content-Type: application/json" \
  -d '{
    "symbols": ["BTC-USD", "ETH-USD"],
    "bar_interval_ms": 250,
    "length_bars": 2000,
    "rf_trees": 100,
    "rf_max_depth": 8,
    "train_split": 0.7,
    "mode": "simulate",
    "starting_cash": 100000
  }'
```

Expected: `{"status":"started"}`

### 5. WebSocket Stream

Connect to WebSocket to receive real-time events:

```bash
# Using wscat (install: npm install -g wscat)
wscat -c ws://127.0.0.1:8080/ws/stream
```

Expected events:
```json
{"Bar":{"ts":"2025-01-05T05:00:00Z","open":100.5,"high":101.2,"low":99.8,"close":100.9,"volume":5432.1,"symbol":"BTC-USD"}}
{"Metrics":{"pnl":245.67,"max_drawdown":0.023,"sharpe":0.0,"win_rate":0.58,"trades":12}}
{"Bar":{"ts":"2025-01-05T05:00:00.250Z","open":100.9,"high":101.5,"low":100.7,"close":101.3,"volume":6021.4,"symbol":"BTC-USD"}}
{"Metrics":{"pnl":312.45,"max_drawdown":0.023,"sharpe":0.0,"win_rate":0.60,"trades":15}}
...
{"Done"}
```

## UI Testing

### 1. Access Station

Open http://127.0.0.1:3000 in your browser.

### 2. Navigate to Assets

Click **Assets** in the navigation.

Expected:
- Empty state message: "No assets available yet. Assets will appear here once exchange connectivity is implemented."

### 3. Start Simulation

Click **Simulate** in the navigation.

Test parameters:
1. Symbols: `BTC-USD,ETH-USD`
2. Bar Interval: `250` ms
3. Length: `2000` bars
4. RF Trees: `100`
5. RF Max Depth: `8`
6. Train/Test Split: `0.7`
7. Starting Cash: `100000`

Click **Start Simulation**.

Expected:
- Button changes to "Simulation Running..."
- Message appears: "After starting, view real-time results in the Dashboard"

### 4. View Dashboard

Click **Dashboard** in the navigation.

Expected:
- **Connection Status**: Green "● Connected"
- **Latest Metrics** card showing:
  - PnL: Dollar amount (e.g., $234.56)
  - Max Drawdown: Percentage (e.g., 2.34%)
  - Win Rate: Percentage (e.g., 58.00%)
  - Trades: Count (e.g., 12)
- **Latest Bar** card showing:
  - Symbol: BTC-USD or ETH-USD
  - Close, High, Low prices
- Metrics updating in real-time (every ~250ms)
- Total bars and metrics counters increasing

## Troubleshooting

### Server won't start

Check if port 8080 is already in use:
```bash
lsof -i :8080
# Or on Linux:
sudo netstat -tulpn | grep 8080
```

Change `SERVER_PORT` in `.env` if needed.

### Station won't start

Check if port 3000 is already in use:
```bash
lsof -i :3000
```

Change `STATION_PORT` in `.env` if needed.

### WebSocket not connecting

Verify:
1. Server is running: `curl http://127.0.0.1:8080/health`
2. CORS is enabled (should be by default)
3. No firewall blocking WebSocket connections
4. Browser console for errors

### No metrics appearing

Possible causes:
1. Simulation not started (check `/api/sim/status`)
2. WebSocket disconnected (check Dashboard connection status)
3. Simulation completed (all bars processed)

Start a new simulation from the Simulate page.

### Database connection errors

Check containers are healthy:
```bash
docker ps --filter name=nexow_
```

If unhealthy, restart:
```bash
cargo run -p nexow -- db down
cargo run -p nexow -- db up
```

Check logs:
```bash
docker logs nexow_postgres_app
docker logs nexow_timescaledb
docker logs nexow_qdrant
```

## Performance Testing

### Benchmark Engine

Test engine performance with different parameters:

```bash
# Fast simulation (250ms intervals, 1000 bars)
# Expected: ~250 seconds total runtime

# Medium simulation (100ms intervals, 5000 bars)
# Expected: ~500 seconds total runtime

# Large simulation (50ms intervals, 10000 bars)
# Expected: ~500 seconds total runtime
```

Monitor:
- CPU usage of server process
- Memory consumption
- WebSocket message throughput
- UI responsiveness

### Database Load

Check database connections:

```bash
# PostgreSQL
docker exec nexow_postgres_app psql -U nexow -d nexow_app -c "SELECT count(*) FROM pg_stat_activity;"

# TimescaleDB
docker exec nexow_timescaledb psql -U nexow -d nexow_ts -c "SELECT count(*) FROM pg_stat_activity;"
```

## Cleanup

### Remove PID files

If processes weren't stopped cleanly:
```bash
rm -rf .nexow/*.pid
```

### Reset databases

To start fresh:
```bash
cargo run -p nexow -- db down
docker volume rm nexow_postgres_app_data nexow_timescale_data nexow_qdrant_data
cargo run -p nexow -- db up
```

### Clean build artifacts

```bash
cargo clean
cd station
rm -rf node_modules .nuxt .output
yarn install
cd ..
```

## Automated Testing

### Rust Tests

```bash
# Run all tests
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test --package nexow-engine --test test_name
```

### Linting

```bash
# Format check
cargo fmt --all -- --check

# Clippy
cargo clippy --workspace --all-targets -- -D warnings
```

### Station Tests

```bash
cd station
yarn test  # (if tests are configured)
yarn lint
yarn build  # Verify build works
```

## CI/CD Testing

GitHub Actions runs on push to `main` branch:

See `.github/workflows/ci.yml` for:
- Rust build, lint, format checks
- Station build and lint (once configured)

## Next Steps

After verifying all manual tests pass:

1. Add exchange connectivity (Phase 2)
2. Implement real data feeds
3. Add comprehensive automated tests
4. Set up end-to-end integration tests
5. Performance benchmarking suite
6. Load testing

---

For issues or questions, open a GitHub Issue or Discussion.

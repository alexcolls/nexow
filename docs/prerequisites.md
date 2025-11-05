# Prerequisites

## Required Tools

You need the following tools installed on your system:

### Rust Toolchain
- **rustup** (Rust toolchain installer)
- **cargo** (Rust package manager)
- **rustc** (Rust compiler - stable channel)

### Node.js & Package Manager
- **Node.js** 18+ 
- **Yarn** (via corepack)

### Docker
- **Docker** 
- **Docker Compose** (Docker CLI with `docker compose` command)

### Version Control
- **Git**

## Verification & Installation

### Check Rust
```bash
rustup --version
rustc --version
cargo --version
```

If not installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Check Node.js & Yarn
```bash
node -v
corepack enable
yarn -v
```

### Check Docker
```bash
docker --version
docker compose version
```

### Check Git
```bash
git --version
```

## Optional Tools

### SQLx CLI (for database migrations)
```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
```

This tool is useful for managing PostgreSQL migrations but not strictly required for development.

## Next Steps

After verifying all prerequisites, proceed to:
1. Copy `.env.sample` to `.env` and configure your environment
2. Run `nexow check` to verify your setup
3. Run `nexow db up` to start databases
4. Run `nexow start` to launch the platform

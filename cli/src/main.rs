use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "nexow")]
#[command(version, about = "Nexow orchestrator CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start Nexow platform (databases, server, station)
    Start {
        /// Skip waiting for health checks
        #[arg(long)]
        no_wait: bool,
    },
    /// Stop Nexow platform processes
    Stop {
        /// Also stop Docker services
        #[arg(long)]
        with_db: bool,
    },
    /// Database service management
    Db {
        #[command(subcommand)]
        cmd: DbCmd,
    },
    /// Check prerequisites and tool versions
    Check,
}

#[derive(Subcommand)]
enum DbCmd {
    /// Start database services
    Up,
    /// Stop database services
    Down,
    /// Show database service status
    Status,
}

const PID_DIR: &str = ".nexow";
const SERVER_PID: &str = ".nexow/server.pid";
const STATION_PID: &str = ".nexow/station.pid";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("nexow=info".parse().unwrap()))
        .with_target(false)
        .init();

    let cli = Cli::parse();
    let root = find_project_root()?;
    env::set_current_dir(&root)?;

    match cli.command {
        Commands::Start { no_wait } => start(no_wait).await?,
        Commands::Stop { with_db } => stop(with_db).await?,
        Commands::Db { cmd } => match cmd {
            DbCmd::Up => db_up().await?,
            DbCmd::Down => db_down().await?,
            DbCmd::Status => db_status().await?,
        },
        Commands::Check => check().await?,
    }

    Ok(())
}

fn find_project_root() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;
    loop {
        if dir.join("Cargo.toml").exists() && dir.join("docker-compose.yml").exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            bail!("Could not find Nexow project root (looking for Cargo.toml and docker-compose.yml)");
        }
    }
}

async fn start(no_wait: bool) -> Result<()> {
    info!("🚀 Starting Nexow platform...");

    // Ensure PID directory exists
    fs::create_dir_all(PID_DIR)?;

    // Load env vars
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".into());
    let station_host = env::var("STATION_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let station_port = env::var("STATION_PORT").unwrap_or_else(|_| "3000".into());

    // 1. Start databases
    info!("📦 Starting database services...");
    run_command("docker", &["compose", "up", "-d"], None)?;

    if !no_wait {
        info!("⏳ Waiting for databases to be healthy...");
        wait_for_docker_health("nexow_postgres_app", 30).await?;
        wait_for_docker_health("nexow_timescaledb", 30).await?;
        wait_for_docker_health("nexow_qdrant", 30).await?;
    }

    // 2. Start server
    info!("🛰️  Starting Nexow server...");
    let server_child = spawn_background(
        "cargo",
        &["run", "--release", "-p", "nexow-server"],
        None,
    )?;
    write_pid(SERVER_PID, server_child.id())?;

    if !no_wait {
        let server_url = format!("http://{}:{}/health", server_host, server_port);
        wait_for_http(&server_url, 30).await?;
        info!("✅ Server healthy at {}", server_url);
    }

    // 3. Start station
    info!("🎨 Starting Nexow station...");
    let station_child = spawn_background("yarn", &["dev"], Some("station"))?;
    write_pid(STATION_PID, station_child.id())?;

    if !no_wait {
        let station_url = format!("http://{}:{}", station_host, station_port);
        wait_for_http(&station_url, 60).await?;
        info!("✅ Station ready at {}", station_url);
    }

    println!();
    info!("🎉 Nexow is running!");
    println!("   Server:  http://{}:{}", server_host, server_port);
    println!("   Station: http://{}:{}", station_host, station_port);
    println!();
    info!("💡 Use 'nexow stop' to shut down");

    Ok(())
}

async fn stop(with_db: bool) -> Result<()> {
    info!("🛑 Stopping Nexow platform...");

    // Stop station
    if let Some(pid) = read_pid(STATION_PID)? {
        info!("Stopping station (PID {})...", pid);
        kill_process(pid);
        let _ = fs::remove_file(STATION_PID);
    } else {
        warn!("Station PID file not found");
    }

    // Stop server
    if let Some(pid) = read_pid(SERVER_PID)? {
        info!("Stopping server (PID {})...", pid);
        kill_process(pid);
        let _ = fs::remove_file(SERVER_PID);
    } else {
        warn!("Server PID file not found");
    }

    // Optionally stop databases
    if with_db {
        info!("Stopping database services...");
        run_command("docker", &["compose", "down"], None)?;
    }

    info!("✅ Nexow stopped");
    Ok(())
}

async fn db_up() -> Result<()> {
    info!("📦 Starting database services...");
    run_command("docker", &["compose", "up", "-d"], None)?;
    info!("✅ Database services starting");
    Ok(())
}

async fn db_down() -> Result<()> {
    info!("📦 Stopping database services...");
    run_command("docker", &["compose", "down"], None)?;
    info!("✅ Database services stopped");
    Ok(())
}

async fn db_status() -> Result<()> {
    info!("📊 Database service status:");
    run_command(
        "docker",
        &["ps", "--filter", "name=nexow_", "--format", "table {{.Names}}\t{{.Status}}\t{{.Ports}}"],
        None,
    )?;
    Ok(())
}

async fn check() -> Result<()> {
    info!("🔍 Checking prerequisites...");
    println!();

    let tools = vec![
        ("docker", &["--version"] as &[&str]),
        ("docker", &["compose", "version"]),
        ("cargo", &["--version"]),
        ("rustc", &["--version"]),
        ("node", &["--version"]),
        ("yarn", &["--version"]),
    ];

    let mut all_ok = true;
    for (tool, args) in tools {
        match which::which(tool) {
            Ok(path) => {
                let output = Command::new(&path).args(args).output();
                match output {
                    Ok(out) if out.status.success() => {
                        let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
                        println!("✅ {}: {}", tool, version);
                    }
                    _ => {
                        println!("⚠️  {}: Found but unable to get version", tool);
                    }
                }
            }
            Err(_) => {
                println!("❌ {}: Not found in PATH", tool);
                all_ok = false;
            }
        }
    }

    println!();
    if all_ok {
        info!("✅ All prerequisites satisfied");
    } else {
        error!("❌ Some prerequisites missing. See docs/prerequisites.md");
        bail!("Prerequisites check failed");
    }

    Ok(())
}

fn run_command(cmd: &str, args: &[&str], cwd: Option<&str>) -> Result<()> {
    let mut command = Command::new(cmd);
    command.args(args);
    if let Some(dir) = cwd {
        command.current_dir(dir);
    }
    let status = command.status().context(format!("Failed to run: {} {}", cmd, args.join(" ")))?;
    if !status.success() {
        bail!("Command failed: {} {}", cmd, args.join(" "));
    }
    Ok(())
}

fn spawn_background(cmd: &str, args: &[&str], cwd: Option<&str>) -> Result<Child> {
    let mut command = Command::new(cmd);
    command
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null());
    if let Some(dir) = cwd {
        command.current_dir(dir);
    }
    command.spawn().context(format!("Failed to spawn: {} {}", cmd, args.join(" ")))
}

fn write_pid(path: &str, pid: u32) -> Result<()> {
    fs::write(path, pid.to_string()).context(format!("Failed to write PID file: {}", path))
}

fn read_pid(path: &str) -> Result<Option<u32>> {
    if !Path::new(path).exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(path)?;
    Ok(content.trim().parse().ok())
}

#[cfg(unix)]
fn kill_process(pid: u32) {
    use std::process::Command;
    let _ = Command::new("kill").arg(pid.to_string()).status();
}

#[cfg(windows)]
fn kill_process(pid: u32) {
    use std::process::Command;
    let _ = Command::new("taskkill")
        .args(&["/F", "/PID", &pid.to_string()])
        .status();
}

async fn wait_for_http(url: &str, timeout_secs: u64) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()?;

    let pb = ProgressBar::new(timeout_secs);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );
    pb.set_message(format!("Waiting for {}", url));

    for _ in 0..timeout_secs {
        match client.get(url).send().await {
            Ok(resp) if resp.status().is_success() => {
                pb.finish_with_message(format!("✅ {} is ready", url));
                return Ok(());
            }
            _ => {
                sleep(Duration::from_secs(1)).await;
                pb.inc(1);
            }
        }
    }

    pb.finish_with_message(format!("❌ Timeout waiting for {}", url));
    bail!("Timeout waiting for HTTP endpoint: {}", url)
}

async fn wait_for_docker_health(container: &str, timeout_secs: u64) -> Result<()> {
    let pb = ProgressBar::new(timeout_secs);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );
    pb.set_message(format!("Waiting for {} health", container));

    for _ in 0..timeout_secs {
        let output = Command::new("docker")
            .args(&["inspect", "--format", "{{.State.Health.Status}}", container])
            .output();

        if let Ok(out) = output {
            let status = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if status == "healthy" {
                pb.finish_with_message(format!("✅ {} is healthy", container));
                return Ok(());
            }
        }

        sleep(Duration::from_secs(1)).await;
        pb.inc(1);
    }

    pb.finish_with_message(format!("❌ Timeout waiting for {}", container));
    bail!("Timeout waiting for container health: {}", container)
}

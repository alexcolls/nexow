use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nexow")]
#[command(version, about = "Nexow orchestrator CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start,
    Stop,
    Db {
        #[command(subcommand)]
        cmd: DbCmd,
    },
    Check,
}

#[derive(Subcommand)]
enum DbCmd {
    Up,
    Down,
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => {
            println!("Starting Nexow...");
            println!("TODO: Implement full orchestration");
        }
        Commands::Stop => {
            println!("Stopping Nexow...");
            println!("TODO: Implement process termination");
        }
        Commands::Db { cmd: DbCmd::Up } => {
            println!("DB up");
            println!("TODO: docker compose up -d");
        }
        Commands::Db { cmd: DbCmd::Down } => {
            println!("DB down");
            println!("TODO: docker compose down");
        }
        Commands::Db { cmd: DbCmd::Status } => {
            println!("DB status");
            println!("TODO: docker ps --filter name=nexow_");
        }
        Commands::Check => {
            println!("Check");
            println!("TODO: Verify prerequisites");
        }
    }

    Ok(())
}

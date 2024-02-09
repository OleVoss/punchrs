mod commands;
mod execute;

use anyhow::Result;
use clap::Parser;
use commands::PunchDirection;
use execute::Execute;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: PunchDirection,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.command.execute()
}

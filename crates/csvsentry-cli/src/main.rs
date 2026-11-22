use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "csvsentry", version, about)]
struct Cli {
    /// Path to the settings JSON file
    #[arg(short, long)]
    settings: PathBuf,
}

fn main() -> Result<()> {
    let _cli = Cli::parse();
    Ok(())
}

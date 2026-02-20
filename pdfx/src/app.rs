use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    cli.dispatch()
}

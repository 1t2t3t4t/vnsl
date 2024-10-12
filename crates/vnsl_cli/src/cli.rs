use clap::Parser;
use clap_derive::Subcommand;

#[derive(Debug, Parser)]
#[command(name = "vnsl")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
#[command()]
pub enum Command {
    /// Compile Vnsl sources
    Compile,
}

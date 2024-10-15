use clap::Parser;
use clap_derive::{Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "vnsl")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Mode {
    Json,
    Csv,
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Compile Vnsl sources
    Compile {
        #[arg(short, long)]
        /// Specify output path of compiled source.
        output: Option<String>,

        #[arg(short, long, default_value_t = Mode::Json)]
        mode: Mode,
    },
}

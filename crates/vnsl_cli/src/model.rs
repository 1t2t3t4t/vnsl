use clap::Parser;
use clap_derive::{Args, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "vnsl")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CompileMode {
    Json,
    Csv,
}

impl ToString for CompileMode {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Compile Vnsl sources
    Compile {
        #[arg(short, long, default_value_t = CompileMode::Json)]
        mode: CompileMode,

        #[command(flatten)]
        options: CompileOptions,
    },
}

#[derive(Debug, Args)]
pub struct CompileOptions {
    #[arg(short, long)]
    /// Specify output path of compiled source.
    output: Option<String>,
}

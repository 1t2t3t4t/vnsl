use clap::Parser;
use model::Cli;

mod command;
mod model;

fn main() -> anyhow::Result<()> {
    let Cli { command } = Cli::parse();

    match command {
        model::Command::Compile { mode, options } => command::handle_compile(mode, options),
    }
}

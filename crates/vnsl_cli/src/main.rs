use clap::Parser;
use cli::Cli;

mod cli;

fn main() {
    let _ = Cli::parse();
}

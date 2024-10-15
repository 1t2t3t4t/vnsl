use clap::Parser;
use cli::Cli;

mod cli;

fn main() {
    let Cli { command } = Cli::parse();

    println!("{:#?}", command);
}

use clap::Parser;
use std::process::exit;
use vegetable_oil::cli::Cli;

fn main() {
    let result = run();
    if let Err(err) = result {
        eprintln!("{err}");
        exit(1)
    }
}

fn run() -> anyhow::Result<()> {
    Cli::parse().run()
}

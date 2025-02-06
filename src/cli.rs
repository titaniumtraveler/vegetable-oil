use clap::{crate_name, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::io;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Subcommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Subcommands {
    Completions { shell: Shell },
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.cmd {
            Subcommands::Completions { shell } => {
                generate(shell, &mut Cli::command(), crate_name!(), &mut io::stdout());
                Ok(())
            }
        }
    }
}

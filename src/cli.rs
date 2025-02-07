use crate::{ops::read, types::CurrentState, util};
use clap::{crate_name, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::{
    ffi::OsStr,
    io,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Subcommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Subcommands {
    Read {
        /// Read paths from `file`.
        /// When set to `-` reads from stdin.
        #[clap(short, long, default_value = "-")]
        file: PathBuf,

        /// Working directory from which to read relative paths from.
        ///
        /// Defaults to the current working directory.
        /// Note that this **has** to be a path pointing to a directory!
        #[clap(short, long)]
        cwd: Option<PathBuf>,
    },
    Completions {
        shell: Shell,
    },
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.cmd {
            Subcommands::Read { file, cwd } => {
                let paths = util::io::read_input_bytes(&file, 0x1000)?;

                let state = read::read_paths(
                    cwd.as_deref(),
                    paths
                        .split(|b| b == &b'\n')
                        .filter(|slice| !slice.is_empty())
                        .map(|slice| Path::new(OsStr::from_bytes(slice))),
                )?;

                util::io::write_json_to_stdout(&CurrentState { current: state })?;
                Ok(())
            }
            Subcommands::Completions { shell } => {
                generate(shell, &mut Cli::command(), crate_name!(), &mut io::stdout());
                Ok(())
            }
        }
    }
}

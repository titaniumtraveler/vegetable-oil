use crate::{ops::read, types::CurrentState};
use clap::{crate_name, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::{
    ffi::OsStr,
    fs::File,
    io::{self, BufWriter, Read},
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
                let paths = if let b"-" = file.as_os_str().as_encoded_bytes() {
                    let mut buf = Vec::with_capacity(0x1000);
                    io::stdin().read_to_end(&mut buf)?;
                    buf
                } else {
                    let mut buf = Vec::with_capacity(0x1000);
                    File::open(file)?.read_to_end(&mut buf)?;
                    buf
                };

                let state = read::read_paths(
                    cwd.as_deref(),
                    paths
                        .split(|b| b == &b'\n')
                        .filter(|slice| !slice.is_empty())
                        .map(|slice| Path::new(OsStr::from_bytes(slice))),
                )?;

                serde_json::to_writer(
                    BufWriter::new(io::stdout().lock()),
                    &CurrentState { current: state },
                )
                .map_err(Into::into)
            }
            Subcommands::Completions { shell } => {
                generate(shell, &mut Cli::command(), crate_name!(), &mut io::stdout());
                Ok(())
            }
        }
    }
}

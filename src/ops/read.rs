use crate::{
    types::{Entry, EntryKind, State},
    util::{DisplayExt, IdProvider},
};
use anyhow::{anyhow, Context};
use nix::{
    errno::Errno,
    fcntl::{open, AtFlags, OFlag},
    sys::stat::{fstatat, Mode, SFlag},
};
use std::path::Path;

pub fn read_paths<'a, I>(cwd: Option<&Path>, paths: I) -> anyhow::Result<State>
where
    I: Iterator<Item = &'a Path>,
{
    let dirfd = match cwd {
        None => None,
        Some(path) => match open(path, OFlag::O_PATH | OFlag::O_DIRECTORY, Mode::empty()) {
            Ok(fd) => Some(fd),
            Err(Errno::ENOTDIR) => {
                return Err(anyhow!(
                    "cwd ({path}) is not a directory",
                    path = path.display()
                ));
            }
            Err(err) => {
                return Err(err).context("opening cwd");
            }
        },
    };

    let mut id = IdProvider { next_id: 1 };
    let paths = paths.into_iter();
    let mut entries = Vec::with_capacity(paths.size_hint().0);

    for path in paths {
        let id = id.next_id();
        let entry_kind = match fstatat(dirfd, path, AtFlags::AT_SYMLINK_NOFOLLOW) {
            Ok(stat) => match SFlag::from_bits_retain(stat.st_mode) & SFlag::S_IFMT {
                SFlag::S_IFREG => EntryKind::File,
                SFlag::S_IFDIR => EntryKind::Dir,

                ft @ (SFlag::S_IFLNK
                | SFlag::S_IFIFO
                | SFlag::S_IFCHR
                | SFlag::S_IFBLK
                | SFlag::S_IFSOCK) => EntryKind::Err {
                    msg: format!("file type `{ft}` not supported", ft = ft.display()),
                },
                unknown => EntryKind::Err {
                    msg: format!("{unknown}", unknown = unknown.display()),
                },
            },
            Err(err) => EntryKind::Err {
                msg: format!("failed to stat file: {err}"),
            },
        };

        entries.push(Entry {
            id,
            path: path.to_path_buf(),
            entry_kind,
        });
    }

    Ok(State(entries))
}

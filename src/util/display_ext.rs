use nix::sys::stat::SFlag;
use std::fmt;

pub(crate) struct Display<T>(T);

pub(crate) trait DisplayExt: Sized {
    fn display(self) -> Display<Self> {
        Display(self)
    }
}

impl DisplayExt for SFlag {}

impl fmt::Display for Display<SFlag> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 & SFlag::S_IFMT {
            SFlag::S_IFSOCK => f.write_str("socket"),
            SFlag::S_IFLNK => f.write_str("symbolic link"),
            SFlag::S_IFREG => f.write_str("regular file"),
            SFlag::S_IFBLK => f.write_str("block device"),
            SFlag::S_IFDIR => f.write_str("directory"),
            SFlag::S_IFCHR => f.write_str("character device"),
            SFlag::S_IFIFO => f.write_str("FIFO"),
            unknown => {
                write!(f, "unknown filetype ({unknown})", unknown = unknown.bits())
            }
        }
    }
}

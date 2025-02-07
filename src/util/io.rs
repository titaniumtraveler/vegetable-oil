use serde::Serialize;
use std::{
    fs::File,
    io::{self, BufWriter, Read},
    os::unix::ffi::OsStrExt,
    path::Path,
};

pub(crate) fn read_input_bytes(path: &Path, capacity: usize) -> io::Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(capacity);

    if let b"-" = path.as_os_str().as_bytes() {
        io::stdin().lock().read_to_end(&mut buf)?;
    } else {
        File::open(path)?.read_to_end(&mut buf)?;
    };

    Ok(buf)
}

pub(crate) fn write_json_to_stdout<T: Serialize>(value: &T) -> serde_json::Result<()> {
    serde_json::to_writer(BufWriter::new(io::stdout().lock()), value)
}

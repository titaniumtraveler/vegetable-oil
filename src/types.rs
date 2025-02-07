use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct State(pub Vec<Entry>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id: u32,
    pub path: PathBuf,
    #[serde(flatten)]
    pub entry_kind: EntryKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "data")]
pub enum EntryKind {
    File,
    Dir,
    Err { msg: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentState {
    pub current: State,
}

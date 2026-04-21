use std::{path::PathBuf, rc::Rc};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type FileId = usize;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(rename = "DoqoSource")]
pub struct Source {
    pub path: PathBuf,
    pub content: Rc<String>,
}
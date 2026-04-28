use std::{path::PathBuf};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type SourceId = usize;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(rename = "DoqoSource")]
pub struct Source {
    pub path: PathBuf,
    pub content: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(rename = "DoqoSpan")]
pub struct Span {
  pub source_id: SourceId,
  pub start: usize,
  pub end: usize,
}
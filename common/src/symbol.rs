use std::{collections::HashMap, fs::{self, File}, path::{Path, PathBuf}, rc::Rc};
use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::{Documentation, LanguagePlugin, source::{SourceId, Source, Span}};

pub type SymbolId = usize;

/// Holds information about a single symbol in the source.
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, rename = "DoqoSymbol")]
pub struct Symbol {
  pub kind: String,
  pub fqid: String,

  pub span: Span,
  pub documentation: Documentation,

  pub parent: Option<SymbolId>,
  pub children: Vec<SymbolId>,
}

impl Symbol {
    pub fn new(name: &str, kind: &str, source_file_id: SourceId, start: usize, end: usize, scope: &[String], comments: &[String]) -> Self {
      Self {
        kind: String::from(kind),
        documentation: Documentation::new(comments),
        parent: None,
        children: Vec::new(),

        span: Span { source_id: source_file_id, start, end },

        fqid: if scope.is_empty() {
          String::from(name)
        } else {
          format!("{}::{}", scope.join("::"), name)
        }
      }
    }

    // FQID mappings

    pub fn name(&self) -> &str {
      self.fqid.rsplit_once("::").map(|(_scope, name)| name).unwrap_or(self.fqid.as_str())
    }

    pub fn scope(&self) -> &str {
      self.fqid.rsplit_once("::").map(|(scope, _name)| scope).unwrap_or("")
    }

    // Comments

    pub fn append_comment(&mut self, comment: &str) {
      self.documentation.append(comment);
    }
}
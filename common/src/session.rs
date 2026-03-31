use std::{collections::{HashMap, HashSet}, error::Error, fs, path::{Path, PathBuf}, rc::Rc, sync::Arc};

use crate::{LanguageRegistry, SymbolTable};

/// A session for multi-language projects
/// 
/// Q: A session fogja a saját fájljain végig hívni a LanguageProcessor extract_symbols függvényét, majd hozzáadni azokat a symbol_table-höz?
pub struct Session {
  /// Symbol table for the session.
  symbol_table: SymbolTable,

  /// Language plugins for the session.
  language_registry: LanguageRegistry,

  /// Maps filenames to their source codes.
  // TODO: what if the project is so big that it maxes out the RAM usage?
  sources: HashMap<PathBuf, String>,

  /// Ignored sources.
  ignored: HashSet<PathBuf>,
}

impl Session {
  pub fn new() -> Self {
    Self {
      symbol_table: SymbolTable::new(),
      language_registry: LanguageRegistry::new(),
      sources: HashMap::new(),
      ignored: HashSet::new(),
      //source_files: Vec::new(),
    }
  }

  /// Add a single source file to the session.
  pub fn add_source<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>> {
    let p = fs::canonicalize(path.as_ref())?;

    // TODO: ignore logic
    /*
    if self.ignored.iter().any(|ign| p.starts_with(ign)) {
      return Ok(());
    }
    */

    let source = fs::read_to_string(&p)?;

    self.sources.insert(p, source);
    Ok(())
  }

  // TODO: ignore logic
  /*
  pub fn ignore_source<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>> {
    let p = fs::canonicalize(path.as_ref())?;
    self.ignored.insert(p);
    Ok(())
  }
  */

  pub fn discover_sources<P: AsRef<Path>>(&mut self, path: P) {
    // TODO
  }

  pub fn extract_symbols(&mut self) {
    // TODO
  }

  pub fn generate_documentation(&mut self) {
    // TODO
  }
}
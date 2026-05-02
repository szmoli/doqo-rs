use std::collections::HashSet;
use crate::LanguageProcessor;

pub type PluginId = usize;

/// Different programming languages can be supported through this trait.
pub trait LanguagePlugin: std::fmt::Debug {
  /// Returns a human readable name of the language (eg. "Rust", "Python", "Elixir")
  fn name(&self) -> &'static str;

  /// Returns a unique ID for the language (eg. "rust", "python", "elixir")
  fn id(&self) -> &'static str;

  /// Returns the file extensions associated with the language (eg. ".rs", ".py", ".ex")
  fn extensions(&self) -> HashSet<&'static str>;

  /// Returns a processor for the language.
  fn processor(&self) -> Box<dyn LanguageProcessor>;
}
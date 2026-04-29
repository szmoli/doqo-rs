use std::collections::HashSet;
use crate::LanguageProcessor;

pub type PluginId = usize;

/// Describes a language.
pub trait LanguagePlugin: std::fmt::Debug {
  /*
    Konzi jegyzetek:
    - kiterjesztések
    - symbol nevek/típusok (treesitter entity - role)
    - nyelv neve
    - language processor (másik trait)
   */

  /// Human readable name of the language (eg. "Rust", "Python", "Elixir")
  fn name(&self) -> &'static str;

  fn id(&self) -> &'static str;

  fn symbol_kinds(&self) -> HashSet<&'static str>;

  /// File extensions associated with the language (eg. ".rs", ".py", ".ex")
  fn extensions(&self) -> HashSet<&'static str>;

  /// Processor for the specific language
  fn processor(&self) -> Box<dyn LanguageProcessor>;
}
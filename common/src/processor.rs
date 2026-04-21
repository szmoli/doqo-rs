use std::{mem::take, path::{PathBuf}};

use crate::{Symbol, SymbolId, SymbolTable, source::{FileId, Source}};
use tree_sitter::{Node, Parser};

pub type NodeHandler = fn(node: Node, source: &str, &mut ProcessingContext) -> bool;

pub struct ProcessingContext<'a> {
  scope: Vec<String>,
  comment_buffer: Vec<String>,
  symbol_table: &'a mut SymbolTable,
  id_scope: Vec<SymbolId>,
  source_file_id: FileId,
}

impl<'a> ProcessingContext<'a> {
  pub fn new(symbol_table: &'a mut SymbolTable, source_file_id: FileId) -> Self {
    Self {
      scope: Vec::new(),
      comment_buffer: Vec::new(),
      symbol_table: symbol_table,
      id_scope: Vec::new(),
      source_file_id: source_file_id,
    }
  }

  // Getters
  pub fn current_source_file(&self) -> (FileId, &Source) {
    let source_file = self.symbol_table.get_file(&self.source_file_id).expect("Source file not found in registry");
    (self.source_file_id, source_file)
  }

  pub fn current_scope(&self) -> &[String] {
    &self.scope
  }

  // Scoping

  pub fn enter_scope(&mut self, id: SymbolId, name: &str) {
    self.scope.push(String::from(name));
    self.id_scope.push(id);
    debug_assert_eq!(self.id_scope.len(), self.scope.len());
  }

  pub fn exit_scope(&mut self) -> Option<(SymbolId, String)> {
    let result = self.id_scope.pop().zip(self.scope.pop());
    debug_assert_eq!(self.id_scope.len(), self.scope.len());
    result
  }

  pub fn current_symbol(&self) -> Option<&Symbol> {
    let id = self.id_scope.last()?;
    self.symbol_table.get_symbol(id)
  }

  pub fn current_symbol_mut(&mut self) -> Option<&mut Symbol> {
    let id = self.id_scope.last()?;
    self.symbol_table.get_symbol_mut(id)
  }

  // Registry

  pub fn register_symbol(&mut self, mut symbol: Symbol) -> SymbolId {
    let parent_id = self.id_scope.last().copied();
    symbol.parent = parent_id;

    let id = self.symbol_table.register_symbol(symbol);

    if let Some(parent_id) = parent_id {
      self.symbol_table.link_child(parent_id, id);
    }

    id
  }

  // Comments

  pub fn push_comment(&mut self, text: &str) {
    self.comment_buffer.push(String::from(text));
  }

  /// Makes a new Documentation from the comment buffer.
  /// 
  /// Side effects: 
  /// - clears the comment buffer.
  pub fn take_comments(&mut self) -> Vec<String> {
    take(&mut self.comment_buffer)
  }
}

/// Processes a specific language into symbols.
pub trait LanguageProcessor {
    /// Get the Tree Sitter grammar for the language.
    fn language(&self) -> tree_sitter::Language;

    /// Takes a path to a source file and processes its contents.
    /// 
    /// Side effects: 
    /// - registers the source file into the symbol table
    /// - registers the symbols found in the file while walking its syntax tree
    fn process(&self, source_path: &PathBuf, symbol_table: &mut SymbolTable) {
        let source_file_id = symbol_table.register_file(source_path.to_path_buf());
        let source_rc = symbol_table.get_source(&source_file_id).expect("Couldn't find file in registry");
        let source = source_rc.as_str();

        let mut parser = Parser::new();
        parser
            .set_language(&self.language())
            .expect("Failed to set parser language.");

        let tree = parser.parse(source, None).expect("Failed to parse tree.");

        let mut context = ProcessingContext::new(symbol_table, source_file_id);

        self.walk_recursive(tree.root_node(), source, &mut context);
    }

    fn walk_recursive(&self, node: Node, source: &str, context: &mut ProcessingContext) {
      let pushed_stack = self.handle_node(node, source, context);

      let mut cursor = node.walk();
      for child in node.named_children(&mut cursor) {
        self.walk_recursive(child, source, context);
      }

      if pushed_stack {
        context.exit_scope();
      }
    }

    fn handle_node(&self, node: Node, source: &str, context: &mut ProcessingContext) -> bool;
}

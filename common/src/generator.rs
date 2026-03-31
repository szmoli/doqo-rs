pub trait DocumentationGenerator {
  fn generate(&self, symbol_table: &SymbolTable) -> Result<(), dyn Error>;
}
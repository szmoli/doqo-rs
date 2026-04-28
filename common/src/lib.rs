pub mod plugin;
pub mod processor;

mod source;
mod documentation;
mod session;
mod symbol;
mod registry;

pub use documentation::{Documentation, Metadata};
pub use plugin::{LanguagePlugin, LanguageRegistry};
pub use processor::LanguageProcessor;
pub use session::Session;
pub use symbol::{Symbol, SymbolId};
pub use registry::Registry;

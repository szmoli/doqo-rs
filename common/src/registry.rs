use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
    rc::Rc,
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    LanguagePlugin, Symbol, SymbolId,
    plugin::PluginId,
    source::{Source, SourceId},
};

/// The symbol table. Maps IDs or FQIDs to Symbols.
/// The current_id starts from 0 and is incremented every time a new symbol is registered.
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, rename = "DoqoRegistry")]
pub struct Registry {
    symbols: HashMap<SymbolId, Symbol>,
    sources: HashMap<SourceId, Rc<Source>>,
    #[serde(skip)]
    #[ts(skip)]
    plugins: HashMap<PluginId, Rc<Box<dyn LanguagePlugin>>>,

    plugin_for_file: HashMap<SourceId, PluginId>,
    plugin_for_extension: HashMap<String, PluginId>,

    #[serde(skip_serializing)]
    #[ts(skip)]
    current_symbol_id: SymbolId,
    #[serde(skip_serializing)]
    #[ts(skip)]
    current_source_id: SourceId,
    #[serde(skip_serializing)]
    #[ts(skip)]
    current_plugin_id: SourceId,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            sources: HashMap::new(),
            plugin_for_file: HashMap::new(),
            plugin_for_extension: HashMap::new(),
            plugins: HashMap::new(),
            current_symbol_id: 0,
            current_source_id: 0,
            current_plugin_id: 0,
        }
    }

    /// Register a new symbol.
    /// Side effect: incements current_id by one.
    pub fn register_symbol(&mut self, symbol: Symbol) -> SymbolId {
        let id = self.current_symbol_id;
        self.symbols.insert(id, symbol);
        self.current_symbol_id += 1;
        id
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn LanguagePlugin>) -> PluginId {
        let id = self.current_plugin_id;

        self.plugin_for_extension.extend(
            plugin
                .extensions()
                .into_iter()
                .map(|extension| (extension.to_string(), id))
        );

        self.plugins.insert(id, Rc::new(plugin));

        self.current_plugin_id += 1;
        id
    }

    pub fn register_source(&mut self, path: PathBuf, plugin_id: PluginId) -> SourceId {
        let id = self.current_source_id;

        let content = fs::read_to_string(&path).expect("Couldn't read file.");
        let friendly_language_name = self
            .plugins
            .get(&plugin_id)
            .map(|p| p.name().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let source = Source {
            path,
            content: content,
            language: friendly_language_name, // denormalization so we can avoid the serialization of the plugins, this saves a lot of headaches
        };

        self.sources.insert(id, Rc::new(source));
        self.plugin_for_file.insert(id, plugin_id);
        self.current_source_id += 1;
        id
    }

    pub fn plugin_for_extension(&self, extension: &str) -> Option<&PluginId> {
        self.plugin_for_extension.get(extension)
    }

    pub fn get_source(&self, file_id: &SourceId) -> Option<Rc<Source>> {
        self.sources.get(file_id).cloned()
    }

    pub fn link_child(&mut self, parent_id: SymbolId, child_id: SymbolId) {
        if let Some(parent) = self.symbols.get_mut(&parent_id) {
            if !parent.children.contains(&child_id) {
                parent.children.push(child_id);
            }
        }
    }

    /// Get the Symbol mapped to the internal ID.
    pub fn get_symbol(&self, id: &SymbolId) -> Option<&Symbol> {
        self.symbols.get(id)
    }

    pub fn get_symbol_mut(&mut self, id: &SymbolId) -> Option<&mut Symbol> {
        self.symbols.get_mut(id)
    }

    pub fn append_comment(&mut self, id: &SymbolId, comment: &str) {
        if let Some(symbol) = self.symbols.get_mut(id) {
            symbol.append_comment(comment);
        }
    }

    pub fn json(&self) -> String {
        //serde_json::to_string(self).unwrap()
        serde_json::to_string_pretty(self).unwrap()
    }
}

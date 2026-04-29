use std::{error::Error, fs, path::PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::WalkDir;

use crate::{LanguagePlugin, Registry, plugin::PluginId, source::SourceId};

/// A session for multi-language projects
pub struct Session {
    /// Registry for the plugins, sources and symbols for one session.
    registry: Registry,

    /// The path to the project's root directory.
    project_root: PathBuf,
    /// Set of ignore patterns for paths not to be discovered.
    ignore_set: GlobSet,
}

impl Session {
    /// Initializes a new session and pre-compiles the ignore set.
    pub fn new(project_root: &str, ignore_patterns: &[&str]) -> Result<Self, Box<dyn Error>> {
        let project_root = fs::canonicalize(project_root)?;
        let ignore_set = Self::load_ignore_set(&project_root, ignore_patterns)?;

        Ok(Self {
            registry: Registry::new(),
            project_root: project_root,
            ignore_set: ignore_set,
        })
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn LanguagePlugin>) -> PluginId {
        self.registry.register_plugin(plugin)
    }

    pub fn scan_sources(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Scanning project at {}", self.project_root.display());

        let mut sources_to_register = Vec::new();
        let walker = WalkDir::new(&self.project_root).into_iter();
        let filtered_entries = walker.filter_entry(|e| !self.is_ignored(&e.path().to_path_buf())); // Prune directories for efficiency

        // Collect sources to register
        for entry in filtered_entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if let Some(id) = self.registry.plugin_for_extension(ext).copied() {
                        println!("Found source: {}", path.display());
                        sources_to_register.push((path.to_path_buf(), id));
                    }
                }
            }
        }

        for (path, id) in sources_to_register {
            self.registry.register_source(path, id);
        }

        println!("Finished scanning project.");

        Ok(())
    }

    pub fn process(&mut self) -> String {
        println!("Processing project");

        let tasks = self.registry.sources_for_plugin();

        for (plugin_id, source_ids) in tasks {
            let plugin = self.registry.get_plugin(&plugin_id);

            if let Some(plugin) = plugin {
                let processor = plugin.processor();

                for source_id in source_ids {
                  println!("Processing source {}", source_id);
                  processor.process(source_id, &mut self.registry);
                }
            }
        }

        println!("Finished processing project");
        
        self.registry.json()
    }

    fn load_ignore_set(
        project_root: &PathBuf,
        ignore_patterns: &[&str],
    ) -> Result<GlobSet, Box<dyn Error>> {
        let mut globset_builder = GlobSetBuilder::new();

        let project_root = fs::canonicalize(project_root)?;
        let ignore_path = project_root.join(".doqoignore");

        if ignore_path.exists() {
            let content = fs::read_to_string(&ignore_path)?;
            for line in content.lines() {
                let trimmed = line.trim();

                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }

                globset_builder.add(Glob::new(trimmed)?);
            }
        }

        for pattern in ignore_patterns {
            globset_builder.add(Glob::new(pattern)?);
        }

        Ok(globset_builder.build()?)
    }

    fn is_ignored(&self, path: &PathBuf) -> bool {
        let relative_path = path.strip_prefix(&self.project_root).unwrap_or(path);
        self.ignore_set.is_match(relative_path)
    }
}

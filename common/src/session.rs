use std::{fs, path::PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::WalkDir;
use anyhow::{Result, Context};

use crate::{LanguagePlugin, Registry, plugin::PluginId};

/// A session for multi-language projects. Orchestrates the processing of sources.
pub struct Session {
    /// Registry for the plugins, sources and symbols for one session.
    registry: Registry,

    /// The path to the project's root directory.
    project_path: PathBuf,
    /// Set of ignore patterns for paths not to be discovered.
    ignore_set: GlobSet,
}

impl Session {
    /// Initializes a new session and pre-compiles the ignore set.
    pub fn new(project_path: PathBuf, ignore_patterns: &[String]) -> Result<Self> {
        let project_path = fs::canonicalize(&project_path).context(format!("Failed to cannonicalize project path: {}.", project_path.display()))?;
        let ignore_set = Self::load_ignore_set(&project_path, ignore_patterns).context("Failed to build ignore pattern set.")?;

        Ok(Self {
            registry: Registry::new(),
            project_path,
            ignore_set: ignore_set,
        })
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn LanguagePlugin>) -> PluginId {
        self.registry.register_plugin(plugin)
    }

    /// Discovers sources inside the project while respecting the .doqoignore file and the specified ignore patterns.
    pub fn scan_sources(&mut self) -> Result<()> {
        log::info!("Scanning project at {}...", self.project_path.display());

        let mut sources_to_register = Vec::new();
        let walker = WalkDir::new(&self.project_path).into_iter();
        let filtered_entries = walker.filter_entry(|e| !self.is_ignored(&e.path().to_path_buf())); // Prune directories for efficiency

        // Collect sources to register
        for entry in filtered_entries {
            let entry = entry.context("Failed to create directory entry")?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if let Some(id) = self.registry.plugin_for_extension(ext).copied() {
                        log::info!("Found source: {}...", path.display());
                        sources_to_register.push((path.to_path_buf(), id));
                    }
                }
            }
        }

        for (path, id) in sources_to_register {
            self.registry.register_source(path, id);
        }

        log::info!("Finished scanning project.");

        Ok(())
    }

    /// Processes the discovered sources into a JSON.
    pub fn process(&mut self) -> String {
        log::info!("Processing project...");

        let tasks = self.registry.sources_for_plugin();

        for (plugin_id, source_ids) in tasks {
            let plugin = self.registry.get_plugin(&plugin_id);

            if let Some(plugin) = plugin {
                let processor = plugin.processor();

                for source_id in source_ids {
                  let source = &self.registry.get_source(&source_id).unwrap(); // This unwrap shouldn't cause a problem, because if we have a source id, then it must be in the registry.
                  log::info!("Processing source: {}...", source.path.display());

                  processor.process(source_id, &mut self.registry);
                }
            }
        }

        log::info!("Finished processing project.");
        
        self.registry.json()
    }

    fn load_ignore_set(
        project_path: &PathBuf,
        ignore_patterns: &[String],
    ) -> Result<GlobSet> {
        let mut globset_builder = GlobSetBuilder::new();

        let project_root = fs::canonicalize(project_path).context(format!("Failed to cannonicalize project path: {}.", project_path.display()))?;
        let ignore_path = project_root.join(".doqoignore");

        if ignore_path.exists() {
            let content = fs::read_to_string(&ignore_path).context(format!("Failed to read {}.", ignore_path.display()))?;
            for line in content.lines() {
                let trimmed = line.trim();

                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }

                globset_builder.add(Glob::new(trimmed).context(format!("Failed to add the pattern \"{}\" to ignore pattern set.", trimmed))?);
            }
        }

        for pattern in ignore_patterns {
            globset_builder.add(Glob::new(pattern).context(format!("Failed to add the pattern \"{}\" to ignore pattern set.", pattern))?);
        }

        Ok(globset_builder.build()?)
    }

    fn is_ignored(&self, path: &PathBuf) -> bool {
        let relative_path = path.strip_prefix(&self.project_path).unwrap_or(path);
        self.ignore_set.is_match(relative_path)
    }
}

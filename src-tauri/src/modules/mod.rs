use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait that all WinShaper modules must implement
pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn is_enabled(&self) -> bool;
    fn enable(&mut self) -> anyhow::Result<()>;
    fn disable(&mut self) -> anyhow::Result<()>;
    fn get_settings(&self) -> HashMap<String, serde_json::Value>;
    fn update_settings(&mut self, settings: HashMap<String, serde_json::Value>) -> anyhow::Result<()>;
}

/// Module manager to coordinate all modules
pub struct ModuleManager {
    modules: HashMap<String, Box<dyn Module>>,
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
    
    pub fn register_module(&mut self, module: Box<dyn Module>) {
        let name = module.name().to_string();
        self.modules.insert(name, module);
    }
    
    pub fn enable_module(&mut self, name: &str) -> anyhow::Result<()> {
        if let Some(module) = self.modules.get_mut(name) {
            module.enable()
        } else {
            Err(anyhow::anyhow!("Module '{}' not found", name))
        }
    }
    
    pub fn disable_module(&mut self, name: &str) -> anyhow::Result<()> {
        if let Some(module) = self.modules.get_mut(name) {
            module.disable()
        } else {
            Err(anyhow::anyhow!("Module '{}' not found", name))
        }
    }
    
    pub fn get_module_settings(&self, name: &str) -> Option<HashMap<String, serde_json::Value>> {
        self.modules.get(name).map(|module| module.get_settings())
    }
    
    pub fn update_module_settings(&mut self, name: &str, settings: HashMap<String, serde_json::Value>) -> anyhow::Result<()> {
        if let Some(module) = self.modules.get_mut(name) {
            module.update_settings(settings)
        } else {
            Err(anyhow::anyhow!("Module '{}' not found", name))
        }
    }
}

// Re-export modules
pub mod process_controller;
pub mod dynamic_split;
pub mod taskbar_customizer;
pub mod mouse_action_mapper;
pub mod clipboard_history;
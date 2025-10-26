use super::Module;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub id: u64,
    pub content: String,
    pub content_type: String,
    pub timestamp: u64,
}

pub struct ClipboardHistory {
    enabled: bool,
    max_items: usize,
    expiry_days: u32,
    history: Vec<ClipboardItem>,
}

impl ClipboardHistory {
    pub fn new() -> Self {
        Self {
            enabled: false,
            max_items: 200,
            expiry_days: 30,
            history: Vec::new(),
        }
    }
    
    pub fn get_history(&self) -> &[ClipboardItem] {
        &self.history
    }
    
    pub fn add_item(&mut self, content: String, content_type: String) -> anyhow::Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
            
        let item = ClipboardItem {
            id: timestamp, // Simple ID for now
            content,
            content_type,
            timestamp,
        };
        
        self.history.insert(0, item);
        
        // Limit history size
        if self.history.len() > self.max_items {
            self.history.truncate(self.max_items);
        }
        
        Ok(())
    }
    
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Module for ClipboardHistory {
    fn name(&self) -> &'static str {
        "clipboard_history"
    }
    
    fn description(&self) -> &'static str {
        "Persistent history with search and quick paste"
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn enable(&mut self) -> anyhow::Result<()> {
        log::info!("Enabling Clipboard History module");
        self.enabled = true;
        // TODO: Setup clipboard monitoring
        Ok(())
    }
    
    fn disable(&mut self) -> anyhow::Result<()> {
        log::info!("Disabling Clipboard History module");
        self.enabled = false;
        // TODO: Stop clipboard monitoring
        Ok(())
    }
    
    fn get_settings(&self) -> HashMap<String, serde_json::Value> {
        let mut settings = HashMap::new();
        settings.insert("max_items".to_string(), serde_json::json!(self.max_items));
        settings.insert("expiry_days".to_string(), serde_json::json!(self.expiry_days));
        settings
    }
    
    fn update_settings(&mut self, settings: HashMap<String, serde_json::Value>) -> anyhow::Result<()> {
        if let Some(max_items) = settings.get("max_items") {
            if let Some(value) = max_items.as_u64() {
                self.max_items = value as usize;
            }
        }
        
        if let Some(expiry_days) = settings.get("expiry_days") {
            if let Some(value) = expiry_days.as_u64() {
                self.expiry_days = value as u32;
            }
        }
        
        Ok(())
    }
}
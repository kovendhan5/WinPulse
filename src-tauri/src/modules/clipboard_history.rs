use super::Module;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use arboard::Clipboard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub id: u64,
    pub content: String,
    pub content_type: String,
    pub timestamp: i64,
    pub preview: String,
}

impl ClipboardItem {
    fn new(content: String, content_type: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let preview = if content.len() > 100 {
            format!("{}...", &content[..100])
        } else {
            content.clone()
        };

        Self {
            id: timestamp as u64,
            content,
            content_type,
            timestamp,
            preview,
        }
    }
}

pub struct ClipboardHistory {
    enabled: bool,
    max_items: usize,
    expiry_days: u32,
    history: Arc<Mutex<Vec<ClipboardItem>>>,
    clipboard: Option<Clipboard>,
    last_content: Arc<Mutex<String>>,
}

impl ClipboardHistory {
    pub fn new() -> Self {
        Self {
            enabled: false,
            max_items: 200,
            expiry_days: 30,
            history: Arc::new(Mutex::new(Vec::new())),
            clipboard: None,
            last_content: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn get_history(&self) -> anyhow::Result<Vec<ClipboardItem>> {
        let history = self.history.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        Ok(history.clone())
    }

    pub fn add_item(&mut self, content: String, content_type: String) -> anyhow::Result<()> {
        // Check if content is different from last item
        let mut last = self.last_content.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        if *last == content {
            return Ok(());
        }
        *last = content.clone();
        drop(last);

        let mut history = self.history.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;

        let item = ClipboardItem::new(content, content_type);
        history.insert(0, item);

        // Limit history size
        if history.len() > self.max_items {
            history.truncate(self.max_items);
        }

        // Remove expired items
        let cutoff_timestamp = Utc::now().timestamp() - (self.expiry_days as i64 * 86400);
        history.retain(|item| item.timestamp > cutoff_timestamp);

        log::info!("Added clipboard item. History size: {}", history.len());

        Ok(())
    }

    pub fn clear_history(&mut self) -> anyhow::Result<()> {
        let mut history = self.history.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        history.clear();
        log::info!("Clipboard history cleared");
        Ok(())
    }

    pub fn get_item(&self, id: u64) -> anyhow::Result<Option<ClipboardItem>> {
        let history = self.history.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        Ok(history.iter().find(|item| item.id == id).cloned())
    }

    pub fn copy_to_clipboard(&mut self, id: u64) -> anyhow::Result<()> {
        let item = self.get_item(id)?;
        
        if let Some(item) = item {
            if let Some(ref mut clipboard) = self.clipboard {
                clipboard.set_text(&item.content)?;
                log::info!("Copied item {} to clipboard", id);
            }
        }

        Ok(())
    }

    pub fn search_history(&self, query: &str) -> anyhow::Result<Vec<ClipboardItem>> {
        let history = self.history.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let query_lower = query.to_lowercase();
        
        let results: Vec<ClipboardItem> = history
            .iter()
            .filter(|item| item.content.to_lowercase().contains(&query_lower))
            .cloned()
            .collect();

        Ok(results)
    }

    fn start_monitoring(&mut self) -> anyhow::Result<()> {
        // Initialize clipboard
        self.clipboard = Some(Clipboard::new()?);
        
        log::info!("Clipboard monitoring started");
        
        // Note: For full clipboard monitoring, we would need to set up a Windows hook
        // or use a separate thread with polling. For now, this is a basic implementation.
        // The actual monitoring would be done via periodic polling from the frontend
        // or by using Windows clipboard chain/notifications.
        
        Ok(())
    }

    fn stop_monitoring(&mut self) {
        self.clipboard = None;
        log::info!("Clipboard monitoring stopped");
    }

    pub fn check_clipboard(&mut self) -> anyhow::Result<()> {
        if let Some(ref mut clipboard) = self.clipboard {
            match clipboard.get_text() {
                Ok(text) => {
                    if !text.is_empty() {
                        self.add_item(text, "text".to_string())?;
                    }
                }
                Err(e) => {
                    // Clipboard might be empty or contain non-text data
                    log::debug!("Could not get clipboard text: {}", e);
                }
            }
        }
        Ok(())
    }

    pub fn save_to_disk(&self) -> anyhow::Result<()> {
        let history = self.history.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        
        let app_data = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find app data directory"))?;
        
        let winshaper_dir = app_data.join("WinShaper");
        std::fs::create_dir_all(&winshaper_dir)?;
        
        let clipboard_file = winshaper_dir.join("clipboard_history.json");
        let json = serde_json::to_string_pretty(&*history)?;
        std::fs::write(clipboard_file, json)?;
        
        log::info!("Clipboard history saved to disk");
        Ok(())
    }

    pub fn load_from_disk(&mut self) -> anyhow::Result<()> {
        let app_data = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find app data directory"))?;
        
        let clipboard_file = app_data.join("WinShaper").join("clipboard_history.json");
        
        if clipboard_file.exists() {
            let json = std::fs::read_to_string(clipboard_file)?;
            let loaded_history: Vec<ClipboardItem> = serde_json::from_str(&json)?;
            
            let mut history = self.history.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
            *history = loaded_history;
            
            log::info!("Clipboard history loaded from disk: {} items", history.len());
        }
        
        Ok(())
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
        
        // Load existing history from disk
        if let Err(e) = self.load_from_disk() {
            log::warn!("Could not load clipboard history: {}", e);
        }
        
        // Start clipboard monitoring
        self.start_monitoring()?;
        
        Ok(())
    }

    fn disable(&mut self) -> anyhow::Result<()> {
        log::info!("Disabling Clipboard History module");
        self.enabled = false;
        
        // Save history before stopping
        self.save_to_disk()?;
        
        // Stop clipboard monitoring
        self.stop_monitoring();
        
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

        log::info!("Clipboard History settings updated");
        Ok(())
    }
}
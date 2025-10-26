use super::Module;
use std::collections::HashMap;

pub struct DynamicSplit {
    enabled: bool,
    layouts: Vec<String>,
}

impl DynamicSplit {
    pub fn new() -> Self {
        Self {
            enabled: false,
            layouts: vec!["60-40".to_string(), "50-50".to_string()],
        }
    }
}

impl Module for DynamicSplit {
    fn name(&self) -> &'static str {
        "dynamic_split"
    }
    
    fn description(&self) -> &'static str {
        "Split screen into custom shapes with hotkey cycling"
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn enable(&mut self) -> anyhow::Result<()> {
        log::info!("Enabling Dynamic Split module");
        self.enabled = true;
        // TODO: Setup window hooks
        Ok(())
    }
    
    fn disable(&mut self) -> anyhow::Result<()> {
        log::info!("Disabling Dynamic Split module");
        self.enabled = false;
        // TODO: Cleanup window hooks
        Ok(())
    }
    
    fn get_settings(&self) -> HashMap<String, serde_json::Value> {
        let mut settings = HashMap::new();
        settings.insert("layouts".to_string(), serde_json::json!(self.layouts));
        settings
    }
    
    fn update_settings(&mut self, settings: HashMap<String, serde_json::Value>) -> anyhow::Result<()> {
        if let Some(layouts) = settings.get("layouts") {
            if let Some(list) = layouts.as_array() {
                self.layouts = list
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }
        }
        Ok(())
    }
}
use super::Module;
use std::collections::HashMap;
use anyhow::Result;
use serde_json::Value;
use std::process::Command;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    keybd_event, KEYEVENTF_KEYUP, VK_SNAPSHOT,
};

#[derive(Debug, Clone)]
pub enum ActionType {
    Screenshot,
    LaunchApp(String),
    WindowLayout(String),
}

pub struct MouseActionMapper {
    enabled: bool,
    sensitivity: u8,
    actions: HashMap<String, String>,
    settings: HashMap<String, Value>,
}

impl MouseActionMapper {
    pub fn new() -> Self {
        let mut actions = HashMap::new();
        actions.insert("middle_click".to_string(), "screenshot".to_string());
        
        Self {
            enabled: false,
            sensitivity: 50,
            actions,
            settings: HashMap::new(),
        }
    }

    /// Take a screenshot using Windows Print Screen key
    pub fn take_screenshot(&self) -> Result<()> {
        unsafe {
            // Simulate Print Screen key press
            keybd_event(VK_SNAPSHOT.0 as u8, 0, Default::default(), 0);
            keybd_event(VK_SNAPSHOT.0 as u8, 0, KEYEVENTF_KEYUP, 0);
        }
        log::info!("Screenshot captured");
        Ok(())
    }

    /// Launch an application by path or command
    pub fn launch_app(&self, app_path: &str) -> Result<()> {
        // Handle common app shortcuts
        let command = match app_path.to_lowercase().as_str() {
            "notepad" => "notepad.exe",
            "calculator" | "calc" => "calc.exe",
            "explorer" => "explorer.exe",
            "cmd" => "cmd.exe",
            "powershell" => "powershell.exe",
            _ => app_path,
        };

        Command::new(command)
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to launch app: {}", e))?;
        
        log::info!("Launched app: {}", command);
        Ok(())
    }

    /// Get available actions
    pub fn get_available_actions(&self) -> Vec<String> {
        vec![
            "screenshot".to_string(),
            "notepad".to_string(),
            "calculator".to_string(),
            "explorer".to_string(),
        ]
    }
}

impl Module for MouseActionMapper {
    fn name(&self) -> &'static str {
        "mouse_action_mapper"
    }
    
    fn description(&self) -> &'static str {
        "Map clicks/gestures to actions (screenshots, app launch)"
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn enable(&mut self) -> anyhow::Result<()> {
        log::info!("Enabling Mouse Action Mapper module");
        self.enabled = true;
        // TODO: Setup mouse hooks
        Ok(())
    }
    
    fn disable(&mut self) -> anyhow::Result<()> {
        log::info!("Disabling Mouse Action Mapper module");
        self.enabled = false;
        // TODO: Cleanup mouse hooks
        Ok(())
    }
    
    fn get_settings(&self) -> HashMap<String, serde_json::Value> {
        self.settings.clone()
    }
    
    fn update_settings(&mut self, settings: HashMap<String, serde_json::Value>) -> anyhow::Result<()> {
        self.settings = settings.into_iter().collect();
        Ok(())
    }
}
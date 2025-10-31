use super::Module;
use std::collections::HashMap;
use anyhow::Result;
use serde_json::Value;

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, ShowWindow, SW_HIDE, SW_SHOW,
};
use windows::core::w;

pub struct TaskbarCustomizer {
    enabled: bool,
    taskbar_visible: bool,
    theme: String,
    opacity: u8,
    settings: HashMap<String, Value>,
}

impl TaskbarCustomizer {
    pub fn new() -> Self {
        Self {
            enabled: false,
            taskbar_visible: true,
            theme: "dark".to_string(),
            opacity: 100,
            settings: HashMap::new(),
        }
    }

    /// Toggle taskbar visibility
    pub fn toggle_taskbar(&mut self) -> Result<()> {
        self.taskbar_visible = !self.taskbar_visible;
        self.set_taskbar_visibility(self.taskbar_visible)
    }

    /// Set taskbar visibility explicitly
    pub fn set_taskbar_visibility(&mut self, visible: bool) -> Result<()> {
        unsafe {
            let hwnd = FindWindowW(w!("Shell_TrayWnd"), None)?;
            if hwnd.0 == 0 {
                return Err(anyhow::anyhow!("Taskbar window not found"));
            }

            let cmd = if visible { SW_SHOW } else { SW_HIDE };
            ShowWindow(hwnd, cmd);
            
            self.taskbar_visible = visible;
            log::info!("Taskbar visibility set to: {}", visible);
            Ok(())
        }
    }

    /// Get current taskbar visibility state
    pub fn is_taskbar_visible(&self) -> bool {
        self.taskbar_visible
    }
}

impl Module for TaskbarCustomizer {
    fn name(&self) -> &'static str {
        "taskbar_customizer"
    }
    
    fn description(&self) -> &'static str {
        "Hide/show elements, resize icons, theme selection"
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn enable(&mut self) -> anyhow::Result<()> {
        log::info!("Enabling Taskbar Customizer module");
        self.enabled = true;
        // TODO: Apply taskbar customizations
        Ok(())
    }
    
    fn disable(&mut self) -> anyhow::Result<()> {
        log::info!("Disabling Taskbar Customizer module");
        self.enabled = false;
        // TODO: Revert taskbar customizations
        Ok(())
    }
    
    fn get_settings(&self) -> HashMap<String, serde_json::Value> {
        let mut settings = HashMap::new();
        settings.insert("theme".to_string(), serde_json::json!(self.theme));
        settings.insert("opacity".to_string(), serde_json::json!(self.opacity));
        settings.insert("taskbar_visible".to_string(), serde_json::json!(self.taskbar_visible));
        settings
    }
    
    fn update_settings(&mut self, settings: HashMap<String, serde_json::Value>) -> anyhow::Result<()> {
        if let Some(theme) = settings.get("theme") {
            if let Some(value) = theme.as_str() {
                self.theme = value.to_string();
            }
        }
        
        if let Some(opacity) = settings.get("opacity") {
            if let Some(value) = opacity.as_u64() {
                self.opacity = value as u8;
            }
        }
        
        Ok(())
    }
}
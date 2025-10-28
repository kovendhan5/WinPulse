use super::Module;
use std::collections::HashMap;
use anyhow::Result;
use serde_json::Value;

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetSystemMetrics, SetWindowPos,
    SM_CXSCREEN, SM_CYSCREEN, SWP_NOZORDER, SWP_SHOWWINDOW,
};

pub struct DynamicSplit {
    enabled: bool,
    layouts: Vec<String>,
    settings: HashMap<String, Value>,
}

impl DynamicSplit {
    pub fn new() -> Self {
        Self {
            enabled: false,
            layouts: vec!["60-40".to_string(), "50-50".to_string()],
            settings: HashMap::new(),
        }
    }

    /// Apply a layout to the currently focused window. Basic MVP layouts are supported.
    pub fn apply_layout(&mut self, layout: &str) -> Result<()> {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.0 == 0 {
                return Err(anyhow::anyhow!("No foreground window"));
            }

            let screen_w = GetSystemMetrics(SM_CXSCREEN);
            let screen_h = GetSystemMetrics(SM_CYSCREEN);

            let (x, y, w, h) = match layout {
                "left" | "50-50" => (0, 0, screen_w / 2, screen_h),
                "right" => (screen_w / 2, 0, screen_w / 2, screen_h),
                "left-60" | "60-40" => (0, 0, (screen_w * 60) / 100, screen_h),
                "right-60" => ((screen_w * 40) / 100, 0, (screen_w * 60) / 100, screen_h),
                "center" => ((screen_w / 8), (screen_h / 8), (screen_w * 3) / 4, (screen_h * 3) / 4),
                _ => return Err(anyhow::anyhow!("Unknown layout")),
            };

            let flags = SWP_NOZORDER | SWP_SHOWWINDOW;
            let ok = SetWindowPos(hwnd, HWND(0), x, y, w, h, flags).as_bool();
            if !ok {
                return Err(anyhow::anyhow!("SetWindowPos failed"));
            }

            Ok(())
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
        Ok(())
    }

    fn disable(&mut self) -> anyhow::Result<()> {
        log::info!("Disabling Dynamic Split module");
        self.enabled = false;
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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

mod modules;
use modules::Module;
use modules::{
    process_controller::{ProcessController, ProcessInfo},
    clipboard_history::{ClipboardHistory, ClipboardItem},
    dynamic_split::DynamicSplit,
    taskbar_customizer::TaskbarCustomizer,
    mouse_action_mapper::MouseActionMapper,
};

// Global state for process controller
struct AppState {
    process_controller: Mutex<ProcessController>,
    clipboard_history: Mutex<ClipboardHistory>,
    dynamic_split: Mutex<DynamicSplit>,
    taskbar_customizer: Mutex<TaskbarCustomizer>,
    mouse_action_mapper: Mutex<MouseActionMapper>,
}

// Configuration structures matching the frontend types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub enabled: bool,
    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinShaperConfig {
    pub modules: ModulesConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulesConfig {
    pub dynamic_split: ModuleConfig,
    pub taskbar_customizer: ModuleConfig,
    pub mouse_action_mapper: ModuleConfig,
    pub process_controller: ModuleConfig,
    pub clipboard_history: ModuleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub start_minimized: bool,
}

impl Default for WinShaperConfig {
    fn default() -> Self {
        let mut dynamic_split_settings = HashMap::new();
        dynamic_split_settings.insert("layouts".to_string(), serde_json::json!(["60-40", "50-50"]));
        
        let mut taskbar_settings = HashMap::new();
        taskbar_settings.insert("theme".to_string(), serde_json::json!("dark"));
        taskbar_settings.insert("opacity".to_string(), serde_json::json!(100));
        
        let mut mouse_settings = HashMap::new();
        mouse_settings.insert("sensitivity".to_string(), serde_json::json!(50));
        
        let mut process_settings = HashMap::new();
        process_settings.insert("threshold_cpu".to_string(), serde_json::json!(80));
        process_settings.insert("threshold_ram".to_string(), serde_json::json!(1024));
        
        let mut clipboard_settings = HashMap::new();
        clipboard_settings.insert("max_items".to_string(), serde_json::json!(200));
        clipboard_settings.insert("expiry_days".to_string(), serde_json::json!(30));

        Self {
            modules: ModulesConfig {
                dynamic_split: ModuleConfig { enabled: false, settings: dynamic_split_settings },
                taskbar_customizer: ModuleConfig { enabled: false, settings: taskbar_settings },
                mouse_action_mapper: ModuleConfig { enabled: false, settings: mouse_settings },
                process_controller: ModuleConfig { enabled: false, settings: process_settings },
                clipboard_history: ModuleConfig { enabled: false, settings: clipboard_settings },
            },
            ui: UiConfig {
                theme: "dark".to_string(),
                start_minimized: true,
            },
        }
    }
}

// Get the configuration file path in %APPDATA%/WinShaper/
fn get_config_path() -> anyhow::Result<PathBuf> {
    let app_data = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find app data directory"))?;
    
    let winshaper_dir = app_data.join("WinShaper");
    
    // Create directory if it doesn't exist
    if !winshaper_dir.exists() {
        fs::create_dir_all(&winshaper_dir)?;
    }
    
    Ok(winshaper_dir.join("config.json"))
}

// Tauri commands
#[tauri::command]
async fn load_config() -> Result<WinShaperConfig, String> {
    match get_config_path() {
        Ok(config_path) => {
            if config_path.exists() {
                match fs::read_to_string(&config_path) {
                    Ok(content) => {
                        match serde_json::from_str::<WinShaperConfig>(&content) {
                            Ok(config) => Ok(config),
                            Err(_) => {
                                // If config is corrupted, return default
                                Ok(WinShaperConfig::default())
                            }
                        }
                    }
                    Err(_) => Ok(WinShaperConfig::default()),
                }
            } else {
                // First time run, return default config
                Ok(WinShaperConfig::default())
            }
        }
        Err(e) => Err(format!("Failed to get config path: {}", e)),
    }
}

#[tauri::command]
async fn save_config(config: WinShaperConfig) -> Result<(), String> {
    match get_config_path() {
        Ok(config_path) => {
            match serde_json::to_string_pretty(&config) {
                Ok(json) => {
                    match fs::write(&config_path, json) {
                        Ok(_) => {
                            log::info!("Configuration saved to {:?}", config_path);
                            Ok(())
                        }
                        Err(e) => Err(format!("Failed to write config file: {}", e)),
                    }
                }
                Err(e) => Err(format!("Failed to serialize config: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to get config path: {}", e)),
    }
}

#[tauri::command]
async fn get_system_info() -> Result<HashMap<String, serde_json::Value>, String> {
    let mut info = HashMap::new();
    
    // Basic system information
    info.insert("platform".to_string(), serde_json::json!("Windows"));
    info.insert("app_version".to_string(), serde_json::json!("0.1.0"));
    
    // Get current process memory usage
    unsafe {
        use windows::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
        use windows::Win32::System::Threading::GetCurrentProcess;
        
        let process = GetCurrentProcess();
        let mut mem_counters: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        mem_counters.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
        
        if GetProcessMemoryInfo(
            process,
            &mut mem_counters,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        ).is_ok() {
            let ram_mb = mem_counters.WorkingSetSize as f64 / (1024.0 * 1024.0);
            info.insert("ram_usage_mb".to_string(), serde_json::json!(ram_mb));
        } else {
            info.insert("ram_usage_mb".to_string(), serde_json::json!(0.0));
        }
    }
    
    info.insert("cpu_usage_percent".to_string(), serde_json::json!(0.5)); // Placeholder
    
    Ok(info)
}

// Module management functions (to be implemented)
#[tauri::command]
async fn enable_module(module_name: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    log::info!("Enabling module: {}", module_name);
    
    match module_name.as_str() {
        "dynamic_split" => {
            let mut ds = state.dynamic_split.lock().map_err(|e| e.to_string())?;
            ds.enable().map_err(|e| e.to_string())
        }
        "taskbar_customizer" => {
            let mut tc = state.taskbar_customizer.lock().map_err(|e| e.to_string())?;
            tc.enable().map_err(|e| e.to_string())
        }
        "mouse_action_mapper" => {
            let mut mam = state.mouse_action_mapper.lock().map_err(|e| e.to_string())?;
            mam.enable().map_err(|e| e.to_string())
        }
        "process_controller" => {
            // Already initialized in app state
            Ok(())
        }
        "clipboard_history" => {
            let mut clipboard = state.clipboard_history.lock().map_err(|e| e.to_string())?;
            clipboard.enable().map_err(|e| e.to_string())
        }
        _ => Err(format!("Unknown module: {}", module_name)),
    }
}

#[tauri::command]
async fn disable_module(module_name: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    log::info!("Disabling module: {}", module_name);
    
    match module_name.as_str() {
        "dynamic_split" => {
            let mut ds = state.dynamic_split.lock().map_err(|e| e.to_string())?;
            ds.disable().map_err(|e| e.to_string())
        }
        "taskbar_customizer" => {
            let mut tc = state.taskbar_customizer.lock().map_err(|e| e.to_string())?;
            tc.disable().map_err(|e| e.to_string())
        }
        "mouse_action_mapper" => {
            let mut mam = state.mouse_action_mapper.lock().map_err(|e| e.to_string())?;
            mam.disable().map_err(|e| e.to_string())
        }
        "clipboard_history" => {
            let mut clipboard = state.clipboard_history.lock().map_err(|e| e.to_string())?;
            clipboard.disable().map_err(|e| e.to_string())
        }
        _ => Ok(())
    }
}

// Process Controller commands
#[tauri::command]
async fn get_running_processes(state: tauri::State<'_, AppState>) -> Result<Vec<ProcessInfo>, String> {
    let mut controller = state.process_controller.lock().map_err(|e| e.to_string())?;
    controller.get_running_processes().map_err(|e| e.to_string())
}

#[tauri::command]
async fn terminate_process(pid: u32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut controller = state.process_controller.lock().map_err(|e| e.to_string())?;
    controller.terminate_process(pid).map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_process_thresholds(state: tauri::State<'_, AppState>) -> Result<Vec<ProcessInfo>, String> {
    let mut controller = state.process_controller.lock().map_err(|e| e.to_string())?;
    controller.check_thresholds().map_err(|e| e.to_string())
}

// Clipboard History commands
#[tauri::command]
async fn get_clipboard_history(state: tauri::State<'_, AppState>) -> Result<Vec<ClipboardItem>, String> {
    let clipboard = state.clipboard_history.lock().map_err(|e| e.to_string())?;
    clipboard.get_history().map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_clipboard(query: String, state: tauri::State<'_, AppState>) -> Result<Vec<ClipboardItem>, String> {
    let clipboard = state.clipboard_history.lock().map_err(|e| e.to_string())?;
    clipboard.search_history(&query).map_err(|e| e.to_string())
}

#[tauri::command]
async fn copy_clipboard_item(id: u64, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut clipboard = state.clipboard_history.lock().map_err(|e| e.to_string())?;
    clipboard.copy_to_clipboard(id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn clear_clipboard_history(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut clipboard = state.clipboard_history.lock().map_err(|e| e.to_string())?;
    clipboard.clear_history().map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_clipboard(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut clipboard = state.clipboard_history.lock().map_err(|e| e.to_string())?;
    clipboard.check_clipboard().map_err(|e| e.to_string())
}

#[tauri::command]
async fn apply_window_layout(layout: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut ds = state.dynamic_split.lock().map_err(|e| e.to_string())?;
    ds.apply_layout(&layout).map_err(|e| e.to_string())
}

#[tauri::command]
async fn cycle_window_layout(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut ds = state.dynamic_split.lock().map_err(|e| e.to_string())?;
    ds.cycle_layout().map_err(|e| e.to_string())
}

#[tauri::command]
async fn toggle_taskbar(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let mut tc = state.taskbar_customizer.lock().map_err(|e| e.to_string())?;
    tc.toggle_taskbar().map_err(|e| e.to_string())?;
    Ok(tc.is_taskbar_visible())
}

#[tauri::command]
async fn get_taskbar_status(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let tc = state.taskbar_customizer.lock().map_err(|e| e.to_string())?;
    Ok(tc.is_taskbar_visible())
}

#[tauri::command]
async fn trigger_screenshot(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mam = state.mouse_action_mapper.lock().map_err(|e| e.to_string())?;
    mam.take_screenshot().map_err(|e| e.to_string())
}

#[tauri::command]
async fn launch_application(app_name: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mam = state.mouse_action_mapper.lock().map_err(|e| e.to_string())?;
    mam.launch_app(&app_name).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            process_controller: Mutex::new(ProcessController::new()),
            clipboard_history: Mutex::new(ClipboardHistory::new()),
            dynamic_split: Mutex::new(DynamicSplit::new()),
            taskbar_customizer: Mutex::new(TaskbarCustomizer::new()),
            mouse_action_mapper: Mutex::new(MouseActionMapper::new()),
        })
        .setup(|app| {
            // Initialize logging
            log::info!("WinShaper starting up...");
            log::info!("Platform: Windows");
            log::info!("Version: 0.1.0 (MVP)");
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_config,
            save_config,
            get_system_info,
            enable_module,
            disable_module,
            get_running_processes,
            terminate_process,
            check_process_thresholds,
            get_clipboard_history,
            search_clipboard,
            copy_clipboard_item,
            clear_clipboard_history,
            check_clipboard,
            apply_window_layout,
            cycle_window_layout,
            toggle_taskbar,
            get_taskbar_status,
            trigger_screenshot,
            launch_application
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';
import { ProcessController } from './components/ProcessController';
import { ClipboardHistory } from './components/ClipboardHistory';
import { DynamicSplit } from './components/DynamicSplit';

// Types for our module configuration
interface ModuleConfig {
  enabled: boolean;
  settings?: Record<string, any>;
}

interface WinShaperConfig {
  modules: {
    dynamic_split: ModuleConfig;
    taskbar_customizer: ModuleConfig;
    mouse_action_mapper: ModuleConfig;
    process_controller: ModuleConfig;
    clipboard_history: ModuleConfig;
  };
  ui: {
    theme: "dark" | "light";
    start_minimized: boolean;
  };
}

function App() {
  const [config, setConfig] = useState<WinShaperConfig>({
    modules: {
      dynamic_split: { enabled: false, settings: { layouts: ["60-40", "50-50"] } },
      taskbar_customizer: { enabled: false, settings: { theme: "dark", opacity: 100 } },
      mouse_action_mapper: { enabled: false, settings: { sensitivity: 50 } },
      process_controller: { enabled: false, settings: { threshold_cpu: 80, threshold_ram: 1024 } },
      clipboard_history: { enabled: false, settings: { max_items: 200, expiry_days: 30 } }
    },
    ui: {
      theme: "dark",
      start_minimized: true
    }
  });

  const [isLoading, setIsLoading] = useState(true);
  const [activeModule, setActiveModule] = useState<string | null>(null);
  const [systemInfo, setSystemInfo] = useState<any>(null);

  // Load configuration on startup
  useEffect(() => {
    loadConfig();
    loadSystemInfo();
    
    // Update system info every 5 seconds
    const interval = setInterval(loadSystemInfo, 5000);
    return () => clearInterval(interval);
  }, []);

  async function loadSystemInfo() {
    try {
      const info = await invoke("get_system_info");
      setSystemInfo(info);
    } catch (error) {
      console.error("Failed to load system info:", error);
    }
  }

  async function loadConfig() {
    try {
      const savedConfig = await invoke<WinShaperConfig>("load_config");
      setConfig(savedConfig);
    } catch (error) {
      console.warn("Failed to load config, using defaults:", error);
    } finally {
      setIsLoading(false);
    }
  }

  async function saveConfig() {
    try {
      await invoke("save_config", { config });
    } catch (error) {
      console.error("Failed to save config:", error);
    }
  }

  function toggleModule(moduleKey: keyof WinShaperConfig["modules"]) {
    setConfig(prev => ({
      ...prev,
      modules: {
        ...prev.modules,
        [moduleKey]: {
          ...prev.modules[moduleKey],
          enabled: !prev.modules[moduleKey].enabled
        }
      }
    }));
  }

  // Save config whenever it changes
  useEffect(() => {
    if (!isLoading) {
      saveConfig();
    }
  }, [config, isLoading]);

  if (isLoading) {
    return (
      <main className="container loading">
        <div className="spinner"></div>
        <p>Loading WinShaper...</p>
      </main>
    );
  }

  return (
    <main className="container">
      <header className="app-header">
        <h1>🔧 WinShaper</h1>
        <p className="subtitle">Windows Customization Hub</p>
      </header>

      {activeModule === 'process_controller' ? (
        <div>
          <button 
            onClick={() => setActiveModule(null)} 
            className="back-btn"
          >
            ← Back to Dashboard
          </button>
          <ProcessController />
        </div>
      ) : activeModule === 'dynamic_split' ? (
        <div>
          <button 
            onClick={() => setActiveModule(null)} 
            className="back-btn"
          >
            ← Back to Dashboard
          </button>
          <DynamicSplit />
        </div>
      ) : activeModule === 'clipboard_history' ? (
        <div>
          <button 
            onClick={() => setActiveModule(null)} 
            className="back-btn"
          >
            ← Back to Dashboard
          </button>
          <ClipboardHistory />
        </div>
      ) : (
        <>
          <div className="modules-grid">
            {/* Dynamic Window Splitting */}
            <div className={`module-card ${config.modules.dynamic_split.enabled ? 'enabled' : ''}`}>
              <div className="module-header">
                <h3>📐 Dynamic Splitting</h3>
                <label className="toggle-switch">
                  <input
                    type="checkbox"
                    checked={config.modules.dynamic_split.enabled}
                    onChange={() => toggleModule('dynamic_split')}
                  />
                  <span className="slider"></span>
                </label>
              </div>
              <p>Split screen into custom shapes (60/40, L/T) with hotkey cycling</p>
              <div className="module-status">
                Status: {config.modules.dynamic_split.enabled ? '🟢 Active' : '⚫ Disabled'}
              </div>
              {config.modules.dynamic_split.enabled && (
                <button 
                  onClick={() => setActiveModule('dynamic_split')}
                  className="view-details-btn"
                >
                  View Details →
                </button>
              )}
            </div>

            {/* Taskbar Customizer */}
            <div className={`module-card ${config.modules.taskbar_customizer.enabled ? 'enabled' : ''}`}>
              <div className="module-header">
                <h3>🎨 Taskbar Customizer</h3>
                <label className="toggle-switch">
                  <input
                    type="checkbox"
                    checked={config.modules.taskbar_customizer.enabled}
                    onChange={() => toggleModule('taskbar_customizer')}
                  />
                  <span className="slider"></span>
                </label>
              </div>
              <p>Hide/show elements, resize icons, theme selection</p>
              <div className="module-status">
                Status: {config.modules.taskbar_customizer.enabled ? '🟢 Active' : '⚫ Disabled'}
              </div>
            </div>

            {/* Mouse Action Mapper */}
            <div className={`module-card ${config.modules.mouse_action_mapper.enabled ? 'enabled' : ''}`}>
              <div className="module-header">
                <h3>🖱️ Mouse Action Mapper</h3>
                <label className="toggle-switch">
                  <input
                    type="checkbox"
                    checked={config.modules.mouse_action_mapper.enabled}
                    onChange={() => toggleModule('mouse_action_mapper')}
                  />
                  <span className="slider"></span>
                </label>
              </div>
              <p>Map clicks/gestures to actions (screenshots, app launch)</p>
              <div className="module-status">
                Status: {config.modules.mouse_action_mapper.enabled ? '🟢 Active' : '⚫ Disabled'}
              </div>
            </div>

            {/* Process Controller */}
            <div className={`module-card ${config.modules.process_controller.enabled ? 'enabled' : ''}`}>
              <div className="module-header">
                <h3>⚡ Process Controller</h3>
                <label className="toggle-switch">
                  <input
                    type="checkbox"
                    checked={config.modules.process_controller.enabled}
                    onChange={() => toggleModule('process_controller')}
                  />
                  <span className="slider"></span>
                </label>
              </div>
              <p>Monitor, suspend, or kill resource-heavy processes</p>
              <div className="module-status">
                Status: {config.modules.process_controller.enabled ? '🟢 Active' : '⚫ Disabled'}
              </div>
              {config.modules.process_controller.enabled && (
                <button 
                  onClick={() => setActiveModule('process_controller')}
                  className="view-details-btn"
                >
                  View Details →
                </button>
              )}
            </div>

            {/* Clipboard History */}
            <div className={`module-card ${config.modules.clipboard_history.enabled ? 'enabled' : ''}`}>
              <div className="module-header">
                <h3>📋 Clipboard History</h3>
                <label className="toggle-switch">
                  <input
                    type="checkbox"
                    checked={config.modules.clipboard_history.enabled}
                    onChange={() => toggleModule('clipboard_history')}
                  />
                  <span className="slider"></span>
                </label>
              </div>
              <p>Persistent history with search and quick paste</p>
              <div className="module-status">
                Status: {config.modules.clipboard_history.enabled ? '🟢 Active' : '⚫ Disabled'}
              </div>
              {config.modules.clipboard_history.enabled && (
                <button 
                  onClick={() => setActiveModule('clipboard_history')}
                  className="view-details-btn"
                >
                  View Details →
                </button>
              )}
            </div>
          </div>

          <footer className="app-footer">
            <div className="system-info">
              <span>💾 RAM Usage: {systemInfo?.ram_usage_mb ? `${systemInfo.ram_usage_mb.toFixed(1)} MB` : '< 20MB'}</span>
              <span>🚀 Status: Running</span>
              <span>📊 {Object.values(config.modules).filter(m => m.enabled).length}/5 Modules Active</span>
            </div>
          </footer>
        </>
      )}
    </main>
  );
}

export default App;

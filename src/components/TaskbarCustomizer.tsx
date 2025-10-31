import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './TaskbarCustomizer.css';

export function TaskbarCustomizer() {
  const [isTaskbarVisible, setIsTaskbarVisible] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [message, setMessage] = useState<string | null>(null);

  useEffect(() => {
    loadTaskbarStatus();
  }, []);

  const loadTaskbarStatus = async () => {
    try {
      const visible = await invoke<boolean>('get_taskbar_status');
      setIsTaskbarVisible(visible);
    } catch (error) {
      console.error('Failed to load taskbar status:', error);
    }
  };

  const handleToggle = async () => {
    setIsLoading(true);
    setMessage(null);
    try {
      const newState = await invoke<boolean>('toggle_taskbar');
      setIsTaskbarVisible(newState);
      setMessage(`✅ Taskbar ${newState ? 'shown' : 'hidden'}`);
    } catch (e: any) {
      setMessage(`❌ Failed: ${e?.toString()}`);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="taskbar-customizer">
      <h2>🎨 Taskbar Customizer</h2>
      <p>Customize Windows taskbar appearance and behavior.</p>

      <div className="control-section">
        <h3>Taskbar Visibility</h3>
        <div className="toggle-control">
          <button 
            onClick={handleToggle} 
            disabled={isLoading}
            className={`toggle-btn ${isTaskbarVisible ? 'visible' : 'hidden'}`}
          >
            {isTaskbarVisible ? '👁️ Taskbar Visible' : '🚫 Taskbar Hidden'}
          </button>
          <p className="status-text">
            Current Status: <strong>{isTaskbarVisible ? 'Visible' : 'Hidden'}</strong>
          </p>
        </div>
      </div>

      <div className="info-box">
        <h4>ℹ️ How to use:</h4>
        <ul>
          <li>Click the button above to toggle taskbar visibility</li>
          <li>Hidden taskbar can be restored anytime</li>
          <li>Changes are immediate and reversible</li>
        </ul>
      </div>

      {message && <div className="message">{message}</div>}
    </div>
  );
}

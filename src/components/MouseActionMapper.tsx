import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './MouseActionMapper.css';

export function MouseActionMapper() {
  const [isExecuting, setIsExecuting] = useState(false);
  const [message, setMessage] = useState<string | null>(null);

  const executeAction = async (action: string, param?: string) => {
    setIsExecuting(true);
    setMessage(null);
    try {
      if (action === 'screenshot') {
        await invoke('trigger_screenshot');
        setMessage('✅ Screenshot captured to clipboard!');
      } else if (action === 'launch') {
        await invoke('launch_application', { appName: param });
        setMessage(`✅ Launched ${param}`);
      }
    } catch (e: any) {
      setMessage(`❌ Failed: ${e?.toString()}`);
    } finally {
      setIsExecuting(false);
    }
  };

  return (
    <div className="mouse-action-mapper">
      <h2>🖱️ Mouse Action Mapper</h2>
      <p>Execute quick actions with mouse clicks and shortcuts.</p>

      <div className="action-section">
        <h3>📸 Screenshot Actions</h3>
        <div className="action-grid">
          <button 
            onClick={() => executeAction('screenshot')} 
            disabled={isExecuting}
            className="action-btn screenshot"
          >
            📷 Take Screenshot
          </button>
          <p className="action-description">
            Captures screen to clipboard (same as Print Screen)
          </p>
        </div>
      </div>

      <div className="action-section">
        <h3>🚀 Quick Launch</h3>
        <div className="action-grid">
          <button 
            onClick={() => executeAction('launch', 'notepad')} 
            disabled={isExecuting}
            className="action-btn launch"
          >
            📝 Notepad
          </button>
          <button 
            onClick={() => executeAction('launch', 'calculator')} 
            disabled={isExecuting}
            className="action-btn launch"
          >
            🔢 Calculator
          </button>
          <button 
            onClick={() => executeAction('launch', 'explorer')} 
            disabled={isExecuting}
            className="action-btn launch"
          >
            📁 File Explorer
          </button>
          <button 
            onClick={() => executeAction('launch', 'cmd')} 
            disabled={isExecuting}
            className="action-btn launch"
          >
            💻 Command Prompt
          </button>
          <button 
            onClick={() => executeAction('launch', 'powershell')} 
            disabled={isExecuting}
            className="action-btn launch"
          >
            ⚡ PowerShell
          </button>
        </div>
      </div>

      <div className="info-box">
        <h4>ℹ️ How to use:</h4>
        <ul>
          <li>Click buttons above to execute actions instantly</li>
          <li>Screenshot is copied to clipboard - paste anywhere</li>
          <li>Quick Launch opens common Windows applications</li>
          <li>Future updates will support custom mouse gestures</li>
        </ul>
      </div>

      {message && <div className="message">{message}</div>}
    </div>
  );
}

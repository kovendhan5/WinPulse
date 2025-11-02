import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './ClipboardHistory.css';

interface ClipboardItem {
  id: number;
  content: string;
  content_type: string;
  timestamp: number;
  preview: string;
}

export function ClipboardHistory() {
  const [items, setItems] = useState<ClipboardItem[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const loadHistory = async () => {
    setIsLoading(true);
    try {
      await invoke('check_clipboard'); // Check for new clipboard content
      const history = await invoke<ClipboardItem[]>('get_clipboard_history');
      setItems(history);
    } catch (error) {
      console.error('Failed to load clipboard history:', error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadHistory();
    const interval = setInterval(loadHistory, 3000); // Check every 3 seconds (optimized)
    return () => clearInterval(interval);
  }, []);

  const handleSearch = async () => {
    if (!searchQuery.trim()) {
      loadHistory();
      return;
    }

    try {
      const results = await invoke<ClipboardItem[]>('search_clipboard', { query: searchQuery });
      setItems(results);
    } catch (error) {
      console.error('Search failed:', error);
    }
  };

  const handleCopy = async (id: number) => {
    try {
      await invoke('copy_clipboard_item', { id });
      alert('Copied to clipboard!');
    } catch (error: any) {
      alert(`Failed to copy: ${error}`);
    }
  };

  const handleClear = async () => {
    if (window.confirm('Are you sure you want to clear all clipboard history?')) {
      try {
        await invoke('clear_clipboard_history');
        await loadHistory();
      } catch (error: any) {
        alert(`Failed to clear history: ${error}`);
      }
    }
  };

  const formatDate = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    if (days > 0) return `${days}d ago`;
    if (hours > 0) return `${hours}h ago`;
    if (minutes > 0) return `${minutes}m ago`;
    return `${seconds}s ago`;
  };

  return (
    <div className="clipboard-history">
      <div className="clipboard-header">
        <h2>📋 Clipboard History</h2>
        <div className="controls">
          <button onClick={loadHistory} disabled={isLoading} className="refresh-btn">
            🔄 {isLoading ? 'Loading...' : 'Refresh'}
          </button>
          <button onClick={handleClear} className="clear-btn">
            🗑️ Clear All
          </button>
        </div>
      </div>

      <div className="search-section">
        <input
          type="text"
          placeholder="Search clipboard history..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
          className="search-input"
        />
        <button onClick={handleSearch} className="search-btn">
          🔍 Search
        </button>
      </div>

      <div className="stats-bar">
        <span className="stat">Total Items: {items.length}</span>
        <span className="stat">Storage: ~{Math.round(items.reduce((sum, item) => sum + item.content.length, 0) / 1024)} KB</span>
      </div>

      <div className="items-container">
        {items.length === 0 && !isLoading && (
          <div className="no-items">
            <p>📋 No clipboard items yet</p>
            <p className="hint">Copy something to get started!</p>
          </div>
        )}

        {items.map((item) => (
          <div key={item.id} className="clipboard-item">
            <div className="item-header">
              <span className="item-type">{item.content_type}</span>
              <span className="item-time">{formatDate(item.timestamp)}</span>
            </div>
            <div className="item-content">
              <pre>{item.preview}</pre>
            </div>
            <div className="item-actions">
              <button
                onClick={() => handleCopy(item.id)}
                className="copy-btn"
                title="Copy to clipboard"
              >
                📄 Copy
              </button>
              <span className="item-length">{item.content.length} chars</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
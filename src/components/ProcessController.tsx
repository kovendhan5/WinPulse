import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './ProcessController.css';

interface ProcessInfo {
  pid: number;
  name: string;
  memory_mb: number;
  cpu_percent: number;
  is_suspended: boolean;
}

export function ProcessController() {
  const [processes, setProcesses] = useState<ProcessInfo[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [sortBy, setSortBy] = useState<'memory' | 'name'>('memory');

  const loadProcesses = async () => {
    setIsLoading(true);
    try {
      const procs = await invoke<ProcessInfo[]>('get_running_processes');
      setProcesses(procs);
    } catch (error) {
      console.error('Failed to load processes:', error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadProcesses();
    const interval = setInterval(loadProcesses, 5000); // Refresh every 5 seconds (optimized)
    return () => clearInterval(interval);
  }, []);

  const handleTerminate = async (pid: number, name: string) => {
    if (window.confirm(`Are you sure you want to terminate ${name}?`)) {
      try {
        await invoke('terminate_process', { pid });
        await loadProcesses();
      } catch (error: any) {
        alert(`Failed to terminate process: ${error}`);
      }
    }
  };

  const sortedProcesses = [...processes].sort((a, b) => {
    if (sortBy === 'memory') {
      return b.memory_mb - a.memory_mb;
    } else {
      return a.name.localeCompare(b.name);
    }
  });

  return (
    <div className="process-controller">
      <div className="controller-header">
        <h2>⚡ Process Controller</h2>
        <div className="controls">
          <button onClick={loadProcesses} disabled={isLoading} className="refresh-btn">
            🔄 {isLoading ? 'Loading...' : 'Refresh'}
          </button>
          <select 
            value={sortBy} 
            onChange={(e) => setSortBy(e.target.value as 'memory' | 'name')}
            className="sort-select"
          >
            <option value="memory">Sort by Memory</option>
            <option value="name">Sort by Name</option>
          </select>
        </div>
      </div>

      <div className="process-stats">
        <div className="stat-card">
          <span className="stat-label">Total Processes</span>
          <span className="stat-value">{processes.length}</span>
        </div>
        <div className="stat-card">
          <span className="stat-label">Total Memory</span>
          <span className="stat-value">
            {Math.round(processes.reduce((sum, p) => sum + p.memory_mb, 0))} MB
          </span>
        </div>
      </div>

      <div className="process-table-container">
        <table className="process-table">
          <thead>
            <tr>
              <th>Process Name</th>
              <th>PID</th>
              <th>Memory (MB)</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {sortedProcesses.map((process) => (
              <tr key={process.pid} className={process.memory_mb > 500 ? 'high-memory' : ''}>
                <td className="process-name">{process.name}</td>
                <td>{process.pid}</td>
                <td className="memory-cell">
                  <div className="memory-bar-container">
                    <div 
                      className="memory-bar" 
                      style={{ 
                        width: `${Math.min((process.memory_mb / 1000) * 100, 100)}%`,
                        backgroundColor: process.memory_mb > 500 ? '#ff4444' : '#00ff88'
                      }}
                    />
                    <span className="memory-value">{Math.round(process.memory_mb)}</span>
                  </div>
                </td>
                <td className="actions-cell">
                  <button
                    onClick={() => handleTerminate(process.pid, process.name)}
                    className="terminate-btn"
                    title="Terminate Process"
                  >
                    ❌
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {processes.length === 0 && !isLoading && (
        <div className="no-processes">
          <p>No processes found</p>
        </div>
      )}
    </div>
  );
}
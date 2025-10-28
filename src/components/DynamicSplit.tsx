import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './DynamicSplit.css';

export function DynamicSplit() {
  const [isApplying, setIsApplying] = useState(false);
  const [message, setMessage] = useState<string | null>(null);

  const apply = async (layout: string) => {
    setIsApplying(true);
    setMessage(null);
    try {
      await invoke('apply_window_layout', { layout });
      setMessage(`Applied layout: ${layout}`);
    } catch (e: any) {
      setMessage(`Failed: ${e?.toString()}`);
    } finally {
      setIsApplying(false);
    }
  };

  return (
    <div className="dynamic-split">
      <h2>📐 Dynamic Window Splitting</h2>
      <p>Apply a layout to the currently focused window.</p>

      <div className="layout-buttons">
        <button onClick={() => apply('left')} disabled={isApplying}>Left (50%)</button>
        <button onClick={() => apply('right')} disabled={isApplying}>Right (50%)</button>
        <button onClick={() => apply('50-50')} disabled={isApplying}>50-50</button>
        <button onClick={() => apply('left-60')} disabled={isApplying}>Left 60-40</button>
        <button onClick={() => apply('right-60')} disabled={isApplying}>Right 60-40</button>
        <button onClick={() => apply('center')} disabled={isApplying}>Center</button>
      </div>

      {message && <div className="message">{message}</div>}
    </div>
  );
}

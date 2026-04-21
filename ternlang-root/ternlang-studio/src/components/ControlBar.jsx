import React, { useState, useEffect } from 'react';

/**
 * ControlBar Component
 * Implementation of the 3-segment ternary execution control pill.
 */
export const ControlBar = () => {
  const [executionState, setExecutionState] = useState(window.executionState || 'idle');

  useEffect(() => {
    const handleStateChange = (e) => {
      setExecutionState(e.detail.state);
    };
    window.addEventListener('executionstatechange', handleStateChange);
    return () => window.removeEventListener('executionstatechange', handleStateChange);
  }, []);

  useEffect(() => {
    if (window.lucide) {
      window.lucide.createIcons();
    }
  }, [executionState]);

  const handlePlay = () => {
    if (executionState === 'paused') {
      window.setExecutionState('running');
      window.lastRealTime = performance.now();
      if (window.currentDriveTimeline) {
        requestAnimationFrame(window.currentDriveTimeline);
      }
    } else if (executionState === 'idle') {
      window.startNewSimulation();
    }
  };

  const handlePause = () => {
    if (executionState === 'running') {
      window.setExecutionState('paused');
    }
  };

  const handleStop = () => {
    window.stopSimulation();
  };

  const segmentStyle = (isActive, color) => ({
    flex: 1,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    height: '100%',
    cursor: 'pointer',
    transition: 'all 0.2s ease',
    background: isActive ? color : 'transparent',
    color: isActive ? '#000' : 'var(--text)',
    borderRight: '1px solid rgba(255,255,255,0.05)',
  });

  return (
    <div className="control-pill-container" style={{
      display: 'inline-flex',
      alignItems: 'center',
      height: '32px',
      width: '140px',
      background: 'var(--bg1)',
      border: '1px solid var(--border2)',
      borderRadius: '16px',
      overflow: 'hidden',
      boxShadow: 'var(--shadow)'
    }}>
      {/* PLAY SEGMENT */}
      <div 
        style={segmentStyle(executionState === 'running', 'var(--green)')}
        onClick={handlePlay}
        title="Play / Resume"
      >
        <i data-lucide="play" style={{ width: '14px', fill: executionState === 'running' ? 'currentColor' : 'none' }}></i>
      </div>

      {/* PAUSE SEGMENT */}
      <div 
        style={segmentStyle(executionState === 'paused', 'var(--amber)')}
        onClick={handlePause}
        title="Pause"
      >
        <i data-lucide="pause" style={{ width: '14px', fill: executionState === 'paused' ? 'currentColor' : 'none' }}></i>
      </div>

      {/* STOP SEGMENT */}
      <div 
        style={{ ...segmentStyle(false, 'transparent'), borderRight: 'none' }}
        onClick={handleStop}
        title="Stop & Reset"
        onMouseEnter={(e) => { e.currentTarget.style.color = 'var(--red)'; }}
        onMouseLeave={(e) => { e.currentTarget.style.color = 'var(--text)'; }}
      >
        <i data-lucide="square" style={{ width: '14px' }}></i>
      </div>
    </div>
  );
};

import React from 'react';
import { useTracerTelemetry } from '../hooks/useTracerTelemetry';

/**
 * TracerView React Component
 * Renders the real-time telemetry stream from the BET VM.
 */
export const TracerView = ({ apiEndpoint }) => {
  const wsUrl = apiEndpoint.replace('http', 'ws') + '/api/tracer/ws';
  const { telemetry, isConnected, clearTelemetry } = useTracerTelemetry(wsUrl);

  return (
    <div className="tracer-container" style={{ padding: '24px', color: 'var(--text)', fontFamily: 'Inter, sans-serif' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
        <h2 style={{ fontSize: '18px', fontWeight: '800', margin: 0 }}>
          <span style={{ color: isConnected ? 'var(--green)' : 'var(--red)', marginRight: '8px' }}>●</span>
          Execution Tracer Pipeline
        </h2>
        <button 
          className="btn btn-ghost" 
          onClick={clearTelemetry}
          style={{ fontSize: '12px' }}
        >
          Clear Trace
        </button>
      </div>

      <div style={{ background: 'var(--bg2)', border: '1px solid var(--border2)', borderRadius: '12px', overflow: 'hidden' }}>
        <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: '12px', textAlign: 'left' }}>
          <thead style={{ background: 'var(--bg3)', color: 'var(--muted)', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
            <tr>
              <th style={{ padding: '12px 16px' }}>Timestamp</th>
              <th style={{ padding: '12px 16px' }}>Node ID</th>
              <th style={{ padding: '12px 16px' }}>Event</th>
              <th style={{ padding: '12px 16px' }}>Result</th>
              <th style={{ padding: '12px 16px' }}>Latency</th>
            </tr>
          </thead>
          <tbody>
            {telemetry.length === 0 ? (
              <tr>
                <td colSpan="5" style={{ padding: '40px', textAlign: 'center', color: 'var(--muted2)', fontStyle: 'italic' }}>
                  Awaiting telemetry firehose...
                </td>
              </tr>
            ) : (
              telemetry.map((event) => {
                const isAffirm = event.signal_out === 1;
                const isReject = event.signal_out === -1;
                const isHold = event.signal_out === 0;
                
                const tritColor = isAffirm ? 'var(--green)' : isReject ? 'var(--red)' : 'var(--amber)';
                const rowOpacity = event.sparse_dropped ? 0.5 : 1;

                return (
                  <tr 
                    key={event.trace_id} 
                    style={{ 
                      borderBottom: '1px solid var(--border)', 
                      opacity: rowOpacity,
                      transition: 'opacity 0.2s ease'
                    }}
                  >
                    <td style={{ padding: '12px 16px', color: 'var(--muted)' }}>
                      {new Date(event.timestamp_ms).toLocaleTimeString()}
                    </td>
                    <td style={{ padding: '12px 16px', fontWeight: '700' }}>{event.node_id}</td>
                    <td style={{ padding: '12px 16px' }}>{event.event_type}</td>
                    <td style={{ padding: '12px 16px' }}>
                      <span style={{ 
                        display: 'inline-block', 
                        padding: '2px 8px', 
                        borderRadius: '4px', 
                        background: `${tritColor}22`, 
                        color: tritColor,
                        fontWeight: '800'
                      }}>
                        {event.signal_out > 0 ? '+1' : event.signal_out < 0 ? '-1' : '0'}
                      </span>
                    </td>
                    <td style={{ padding: '12px 16px' }}>
                      {event.latency_ms}ms
                      {event.sparse_dropped && (
                        <span className="badge" style={{ 
                          marginLeft: '8px', 
                          background: 'var(--cyan)', 
                          color: 'var(--bg)', 
                          fontSize: '9px', 
                          padding: '2px 6px', 
                          borderRadius: '4px',
                          fontWeight: '900'
                        }}>
                          BYPASSED
                        </span>
                      )}
                    </td>
                  </tr>
                );
              })
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};

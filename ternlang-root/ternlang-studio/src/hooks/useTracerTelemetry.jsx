import { useState, useEffect } from 'react';

/**
 * Hook to manage real-time Tracer telemetry from the Rust BET VM.
 * Maintains a chronological log of up to 1000 events.
 */
export function useTracerTelemetry(wsUrl) {
  const [telemetry, setTelemetry] = useState([]);
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    if (!wsUrl) return;

    const socket = new WebSocket(wsUrl);

    socket.onopen = () => {
      console.log('[Tracer WS] Handshake successful. Listening for VM telemetry.');
      setIsConnected(true);
    };

    socket.onmessage = (event) => {
      console.log('[Tracer WS] Payload received:', event.data);
      try {
        if (event.data === 'connected') return;
        const data = JSON.parse(event.data);
        
        setTelemetry((prev) => {
          const updated = [data, ...prev];
          return updated.slice(0, 1000);
        });
      } catch (err) {
        console.error('Failed to parse telemetry event:', err);
      }
    };

    socket.onclose = () => {
      console.warn('Tracer WebSocket disconnected');
      setIsConnected(false);
    };

    socket.onerror = (err) => {
      console.error('[Tracer WS] Connection failed.', err);
    };

    return () => {
      socket.close();
    };
  }, [wsUrl]);

  return { telemetry, isConnected, clearTelemetry: () => setTelemetry([]) };
}

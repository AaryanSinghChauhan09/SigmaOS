import { useState, useEffect } from "react";

export interface NetworkState { supported: boolean; online: boolean; effectiveType: string; downlink: number; rtt: number; saveData: boolean; }

const getConnectionInfo = (): Partial<NetworkState> => {
  const conn = (navigator as any).connection || (navigator as any).mozConnection || (navigator as any).webkitConnection;
  if (!conn) return { supported: false };
  return { supported: true, effectiveType: conn.effectiveType || "unknown", downlink: conn.downlink || 0, rtt: conn.rtt || 0, saveData: conn.saveData || false };
};

export const useNetworkDriver = (): NetworkState => {
  const [state, setState] = useState<NetworkState>({ supported: false, online: navigator.onLine, effectiveType: "4g", downlink: 0, rtt: 0, saveData: false, ...getConnectionInfo() });
  useEffect(() => {
    const update = () => setState(prev => ({ ...prev, online: navigator.onLine, ...getConnectionInfo() }));
    window.addEventListener("online", update); window.addEventListener("offline", update);
    const conn = (navigator as any).connection;
    if (conn) conn.addEventListener("change", update);
    return () => { window.removeEventListener("online", update); window.removeEventListener("offline", update); if (conn) conn.removeEventListener("change", update); };
  }, []);
  return state;
};

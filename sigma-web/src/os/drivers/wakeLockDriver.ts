import { useState, useEffect, useRef } from "react";

export interface WakeLockState { supported: boolean; active: boolean; }

export const useWakeLockDriver = () => {
  const [state, setState] = useState<WakeLockState>({ supported: "wakeLock" in navigator, active: false });
  const lockRef = useRef<any>(null);
  const wantedRef = useRef(false);
  const acquire = async () => {
    if (!("wakeLock" in navigator)) return;
    try {
      lockRef.current = await (navigator as any).wakeLock.request("screen");
      lockRef.current.addEventListener("release", () => { setState(prev=>({...prev,active:false})); if(wantedRef.current) acquire(); });
      wantedRef.current = true; setState(prev=>({...prev,active:true}));
    } catch {}
  };
  const release = async () => { wantedRef.current=false; try { await lockRef.current?.release(); } catch {} lockRef.current=null; setState(prev=>({...prev,active:false})); };
  useEffect(() => {
    const onVisibility = () => { if(document.visibilityState==="visible"&&wantedRef.current) acquire(); };
    document.addEventListener("visibilitychange", onVisibility);
    return () => { document.removeEventListener("visibilitychange", onVisibility); release(); };
  }, []);
  return { ...state, acquire, release };
};

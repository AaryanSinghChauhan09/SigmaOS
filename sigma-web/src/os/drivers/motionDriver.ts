import { useState, useEffect } from "react";

export interface MotionState { supported: boolean; permission: "idle"|"granted"|"denied"|"requesting"; acceleration: { x:number; y:number; z:number }|null; rotationRate: { alpha:number; beta:number; gamma:number }|null; orientation: { alpha:number; beta:number; gamma:number }|null; }

export const useMotionDriver = () => {
  const [state, setState] = useState<MotionState>({ supported: typeof DeviceMotionEvent !== "undefined", permission: "idle", acceleration: null, rotationRate: null, orientation: null });
  const requestPermission = async () => {
    setState(prev => ({ ...prev, permission: "requesting" }));
    try {
      const DME = DeviceMotionEvent as any;
      if (typeof DME.requestPermission === "function") { const result = await DME.requestPermission(); if (result !== "granted") { setState(prev => ({ ...prev, permission: "denied" })); return; } }
      setState(prev => ({ ...prev, permission: "granted" }));
    } catch { setState(prev => ({ ...prev, permission: "denied" })); }
  };
  useEffect(() => {
    if (state.permission !== "granted") return;
    const handleMotion = (e: DeviceMotionEvent) => setState(prev => ({ ...prev, acceleration: e.acceleration ? { x: e.acceleration.x??0, y: e.acceleration.y??0, z: e.acceleration.z??0 } : null, rotationRate: e.rotationRate ? { alpha: e.rotationRate.alpha??0, beta: e.rotationRate.beta??0, gamma: e.rotationRate.gamma??0 } : null }));
    const handleOrientation = (e: DeviceOrientationEvent) => setState(prev => ({ ...prev, orientation: { alpha: e.alpha??0, beta: e.beta??0, gamma: e.gamma??0 } }));
    window.addEventListener("devicemotion", handleMotion); window.addEventListener("deviceorientation", handleOrientation);
    return () => { window.removeEventListener("devicemotion", handleMotion); window.removeEventListener("deviceorientation", handleOrientation); };
  }, [state.permission]);
  return { ...state, requestPermission };
};

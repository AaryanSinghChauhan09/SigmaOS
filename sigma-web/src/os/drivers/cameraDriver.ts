import { useState, useEffect } from "react";
import React from "react";

export interface CameraState { supported: boolean; permission: "idle"|"granted"|"denied"|"requesting"; stream: MediaStream|null; devices: MediaDeviceInfo[]; activeDeviceId: string|null; facingMode: "user"|"environment"; }

export const useCameraDriver = () => {
  const [state, setState] = useState<CameraState>({ supported: !!(navigator.mediaDevices?.getUserMedia), permission: "idle", stream: null, devices: [], activeDeviceId: null, facingMode: "user" });
  const listDevices = async () => { try { const all = await navigator.mediaDevices.enumerateDevices(); setState(prev => ({ ...prev, devices: all.filter(d=>d.kind==="videoinput") })); } catch {} };
  const requestCamera = async (deviceId?: string) => {
    setState(prev => ({ ...prev, permission: "requesting" }));
    try {
      state.stream?.getTracks().forEach(t=>t.stop());
      const stream = await navigator.mediaDevices.getUserMedia({ video: deviceId ? { deviceId: { exact: deviceId } } : { facingMode: state.facingMode } });
      await listDevices();
      setState(prev => ({ ...prev, permission: "granted", stream, activeDeviceId: deviceId??null }));
    } catch { setState(prev => ({ ...prev, permission: "denied" })); }
  };
  const stopCamera = () => { state.stream?.getTracks().forEach(t=>t.stop()); setState(prev => ({ ...prev, stream: null, permission: "idle" })); };
  const switchCamera = () => { const next = state.facingMode==="user"?"environment":"user"; setState(prev=>({...prev,facingMode:next})); requestCamera(); };
  const takeSnapshot = (videoRef: React.RefObject<HTMLVideoElement|null>): string|null => { if (!videoRef.current) return null; const c=document.createElement("canvas"); c.width=videoRef.current.videoWidth; c.height=videoRef.current.videoHeight; c.getContext("2d")?.drawImage(videoRef.current,0,0); return c.toDataURL("image/png"); };
  useEffect(() => () => { state.stream?.getTracks().forEach(t=>t.stop()); }, []);
  return { ...state, requestCamera, stopCamera, switchCamera, takeSnapshot, listDevices };
};

import { useState, useRef, useEffect } from "react";

export interface AudioState { supported: boolean; permission: "idle"|"granted"|"denied"|"requesting"; context: AudioContext|null; micStream: MediaStream|null; frequencyData: Uint8Array|null; volume: number; }

export const useAudioDriver = () => {
  const [state, setState] = useState<AudioState>({ supported: "AudioContext" in window || "webkitAudioContext" in window, permission: "idle", context: null, micStream: null, frequencyData: null, volume: 0 });
  const analyserRef = useRef<AnalyserNode|null>(null);
  const rafRef = useRef<number|null>(null);
  const requestMicrophone = async () => {
    setState(prev => ({ ...prev, permission: "requesting" }));
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const AC = (window as any).AudioContext || (window as any).webkitAudioContext;
      const ctx = new AC(); const source = ctx.createMediaStreamSource(stream); const analyser = ctx.createAnalyser(); analyser.fftSize = 256; source.connect(analyser); analyserRef.current = analyser;
      setState(prev => ({ ...prev, permission: "granted", context: ctx, micStream: stream, frequencyData: new Uint8Array(analyser.frequencyBinCount) }));
      const tick = () => { if (!analyserRef.current) return; const d = new Uint8Array(analyserRef.current.frequencyBinCount); analyserRef.current.getByteFrequencyData(d); setState(prev => ({ ...prev, frequencyData: d, volume: d.reduce((a,b)=>a+b,0)/d.length/255 })); rafRef.current = requestAnimationFrame(tick); };
      rafRef.current = requestAnimationFrame(tick);
    } catch { setState(prev => ({ ...prev, permission: "denied" })); }
  };
  const stopMicrophone = () => { if (rafRef.current) cancelAnimationFrame(rafRef.current); state.micStream?.getTracks().forEach(t=>t.stop()); state.context?.close(); analyserRef.current = null; setState(prev => ({ ...prev, permission: "idle", context: null, micStream: null, frequencyData: null, volume: 0 })); };
  const playBeep = (freq=440, duration=0.3) => { try { const AC = (window as any).AudioContext||(window as any).webkitAudioContext; const ctx=new AC(); const osc=ctx.createOscillator(); const gain=ctx.createGain(); osc.connect(gain); gain.connect(ctx.destination); osc.frequency.value=freq; gain.gain.setValueAtTime(0.3,ctx.currentTime); gain.gain.exponentialRampToValueAtTime(0.001,ctx.currentTime+duration); osc.start(); osc.stop(ctx.currentTime+duration); } catch {} };
  useEffect(() => () => { if (rafRef.current) cancelAnimationFrame(rafRef.current); state.micStream?.getTracks().forEach(t=>t.stop()); }, []);
  return { ...state, requestMicrophone, stopMicrophone, playBeep };
};

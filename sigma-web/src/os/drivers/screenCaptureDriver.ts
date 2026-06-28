import { useState, useRef } from "react";

export interface ScreenCaptureState { supported: boolean; stream: MediaStream|null; recording: boolean; recordedChunks: Blob[]; }

export const useScreenCaptureDriver = () => {
  const [state, setState] = useState<ScreenCaptureState>({ supported: !!(navigator.mediaDevices && (navigator.mediaDevices as any).getDisplayMedia), stream: null, recording: false, recordedChunks: [] });
  const recorderRef = useRef<MediaRecorder|null>(null);
  const startCapture = async () => { try { state.stream?.getTracks().forEach(t=>t.stop()); const stream = await (navigator.mediaDevices as any).getDisplayMedia({ video: true, audio: true }); stream.getTracks()[0].addEventListener("ended", () => setState(prev=>({...prev,stream:null,recording:false}))); setState(prev=>({...prev,stream,recordedChunks:[]})); } catch {} };
  const stopCapture = () => { state.stream?.getTracks().forEach(t=>t.stop()); recorderRef.current?.stop(); setState(prev=>({...prev,stream:null,recording:false})); };
  const startRecording = () => { if (!state.stream) return; const recorder = new MediaRecorder(state.stream); recorderRef.current=recorder; const chunks: Blob[] = []; recorder.ondataavailable = e => { if(e.data.size>0) chunks.push(e.data); }; recorder.onstop = () => setState(prev=>({...prev,recordedChunks:chunks,recording:false})); recorder.start(); setState(prev=>({...prev,recording:true,recordedChunks:[]})); };
  const stopRecording = () => recorderRef.current?.stop();
  const downloadRecording = () => { if(!state.recordedChunks.length) return; const blob=new Blob(state.recordedChunks,{type:"video/webm"}); const url=URL.createObjectURL(blob); const a=document.createElement("a"); a.href=url; a.download=`sigmaos-capture-${Date.now()}.webm`; a.click(); URL.revokeObjectURL(url); };
  return { ...state, startCapture, stopCapture, startRecording, stopRecording, downloadRecording };
};

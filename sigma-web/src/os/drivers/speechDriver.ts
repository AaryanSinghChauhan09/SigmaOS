import { useState, useEffect, useRef } from "react";

export interface SpeechState { supported: boolean; ttsSupported: boolean; listening: boolean; transcript: string; finalTranscript: string; error: string|null; voices: SpeechSynthesisVoice[]; }

export const useSpeechDriver = () => {
  const SR = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
  const [state, setState] = useState<SpeechState>({ supported: !!SR, ttsSupported: "speechSynthesis" in window, listening: false, transcript: "", finalTranscript: "", error: null, voices: [] });
  const recognitionRef = useRef<any>(null);
  useEffect(() => {
    if (!("speechSynthesis" in window)) return;
    const loadVoices = () => setState(prev => ({ ...prev, voices: window.speechSynthesis.getVoices() }));
    loadVoices(); window.speechSynthesis.onvoiceschanged = loadVoices;
    return () => { window.speechSynthesis.onvoiceschanged = null; };
  }, []);
  const startListening = () => {
    if (!SR) return;
    const recognition = new SR(); recognitionRef.current = recognition;
    recognition.continuous = true; recognition.interimResults = true; recognition.lang = "en-US";
    recognition.onresult = (e: any) => { let interim="",final=""; for(let i=e.resultIndex;i<e.results.length;i++){if(e.results[i].isFinal)final+=e.results[i][0].transcript;else interim+=e.results[i][0].transcript;} setState(prev=>({...prev,transcript:interim,finalTranscript:prev.finalTranscript+final})); };
    recognition.onerror = (e: any) => setState(prev=>({...prev,error:e.error,listening:false}));
    recognition.onend = () => setState(prev=>({...prev,listening:false}));
    recognition.start(); setState(prev=>({...prev,listening:true,transcript:"",error:null}));
  };
  const stopListening = () => { recognitionRef.current?.stop(); setState(prev=>({...prev,listening:false})); };
  const clearTranscript = () => setState(prev=>({...prev,transcript:"",finalTranscript:""}));
  const speak = (text: string, voiceIndex=0) => { if (!window.speechSynthesis) return; window.speechSynthesis.cancel(); const u = new SpeechSynthesisUtterance(text); if(state.voices[voiceIndex]) u.voice=state.voices[voiceIndex]; window.speechSynthesis.speak(u); };
  return { ...state, startListening, stopListening, clearTranscript, speak };
};

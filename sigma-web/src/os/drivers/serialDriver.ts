import { useState, useRef } from "react";

export interface SerialState { supported: boolean; ports: Array<{portId:string}>; connected: boolean; receivedData: string; error: string|null; }

export const useSerialDriver = () => {
  const [state, setState] = useState<SerialState>({ supported: "serial" in navigator, ports: [], connected: false, receivedData: "", error: null });
  const portRef = useRef<any>(null);
  const readerRef = useRef<any>(null);
  const readLoop = async (port: any) => {
    while (port.readable) {
      const reader = port.readable.getReader(); readerRef.current = reader;
      try { while(true) { const {value,done}=await reader.read(); if(done) break; setState(prev=>({...prev,receivedData:prev.receivedData+new TextDecoder().decode(value)})); } } catch { break; } finally { reader.releaseLock(); }
    }
  };
  const requestPort = async () => { if(!("serial" in navigator)) return; try { const port = await (navigator as any).serial.requestPort(); await port.open({baudRate:9600}); portRef.current=port; setState(prev=>({...prev,connected:true,error:null})); readLoop(port); } catch(e:any) { setState(prev=>({...prev,error:e.message})); } };
  const send = async (data: string) => { if(!portRef.current?.writable) return; const w=portRef.current.writable.getWriter(); await w.write(new TextEncoder().encode(data)); w.releaseLock(); };
  const disconnect = async () => { try { readerRef.current?.cancel(); await portRef.current?.close(); } catch {} portRef.current=null; setState(prev=>({...prev,connected:false})); };
  return { ...state, requestPort, send, disconnect };
};

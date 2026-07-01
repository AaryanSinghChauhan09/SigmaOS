import { useState } from "react";

export interface MidiPortInfo { id: string; name: string; manufacturer: string; }
export interface MidiMessage { channel: number; command: number; note: number; velocity: number; }
export interface MidiState { supported: boolean; permission: "idle"|"granted"|"denied"; inputs: MidiPortInfo[]; outputs: MidiPortInfo[]; lastMessage: MidiMessage|null; }

export const useMidiDriver = () => {
  const [state, setState] = useState<MidiState>({ supported: "requestMIDIAccess" in navigator, permission: "idle", inputs: [], outputs: [], lastMessage: null });
  let midiAccessRef: any = null;
  const requestAccess = async () => {
    if (!("requestMIDIAccess" in navigator)) return;
    try {
      midiAccessRef = await (navigator as any).requestMIDIAccess();
      const inputs: MidiPortInfo[] = []; const outputs: MidiPortInfo[] = [];
      midiAccessRef.inputs.forEach((port: any) => {
        inputs.push({ id:port.id, name:port.name, manufacturer:port.manufacturer });
        port.onmidimessage = (e: any) => { const [status,note,velocity]=e.data; setState(prev=>({...prev,lastMessage:{channel:status&0x0f,command:status&0xf0,note,velocity}})); };
      });
      midiAccessRef.outputs.forEach((port: any) => outputs.push({ id:port.id, name:port.name, manufacturer:port.manufacturer }));
      setState(prev=>({...prev,permission:"granted",inputs,outputs}));
    } catch { setState(prev=>({...prev,permission:"denied"})); }
  };
  const sendNote = (outputId: string, note: number, velocity=127, durationMs=500) => {
    if (!midiAccessRef) return;
    const output = midiAccessRef.outputs.get(outputId); if(!output) return;
    output.send([0x90,note,velocity]); setTimeout(()=>output.send([0x80,note,0]),durationMs);
  };
  return { ...state, requestAccess, sendNote };
};

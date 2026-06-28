import { useState } from "react";

export interface ClipboardState { supported: boolean; lastCopied: string|null; lastRead: string|null; error: string|null; }

export const useClipboardDriver = () => {
  const [state, setState] = useState<ClipboardState>({ supported: !!navigator.clipboard, lastCopied: null, lastRead: null, error: null });
  const copy = async (text: string) => { try { await navigator.clipboard.writeText(text); setState(prev=>({...prev,lastCopied:text,error:null})); return true; } catch(e: any) { setState(prev=>({...prev,error:e.message})); return false; } };
  const paste = async (): Promise<string|null> => { try { const text = await navigator.clipboard.readText(); setState(prev=>({...prev,lastRead:text,error:null})); return text; } catch(e: any) { setState(prev=>({...prev,error:e.message})); return null; } };
  return { ...state, copy, paste };
};

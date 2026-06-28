import { useState, useEffect } from "react";

export interface OpfsState { supported: boolean; quota: number|null; usage: number|null; files: string[]; loading: boolean; }

export const useOpfsDriver = () => {
  const [state, setState] = useState<OpfsState>({ supported: "storage" in navigator && "getDirectory" in navigator.storage, quota: null, usage: null, files: [], loading: false });
  const getRoot = async () => { try { return await navigator.storage.getDirectory(); } catch { return null; } };
  const refreshFiles = async () => { const root = await getRoot(); if (!root) return; const names: string[] = []; for await (const [name] of (root as any).entries()) names.push(name); setState(prev => ({ ...prev, files: names })); };
  const refreshEstimate = async () => { try { const est = await navigator.storage.estimate(); setState(prev => ({ ...prev, quota: est.quota??null, usage: est.usage??null })); } catch {} };
  useEffect(() => { if (!state.supported) return; refreshEstimate(); refreshFiles(); }, [state.supported]);
  const readFile = async (name: string): Promise<string|null> => { const root = await getRoot(); if (!root) return null; try { const fh = await root.getFileHandle(name); return await (await fh.getFile()).text(); } catch { return null; } };
  const writeFile = async (name: string, content: string): Promise<boolean> => { const root = await getRoot(); if (!root) return false; try { const fh = await root.getFileHandle(name, { create: true }); const w = await (fh as any).createWritable(); await w.write(content); await w.close(); await refreshFiles(); await refreshEstimate(); return true; } catch { return false; } };
  const deleteFile = async (name: string): Promise<boolean> => { const root = await getRoot(); if (!root) return false; try { await root.removeEntry(name); await refreshFiles(); return true; } catch { return false; } };
  return { ...state, readFile, writeFile, deleteFile, refreshFiles };
};

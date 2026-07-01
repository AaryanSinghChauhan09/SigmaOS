import { useState } from "react";

export interface FsFile { name: string; kind: "file"|"directory"; size?: number; }
export interface FileSystemState { supported: boolean; directoryHandle: FileSystemDirectoryHandle|null; files: FsFile[]; currentPath: string; loading: boolean; error: string|null; }

export const useFilesystemDriver = () => {
  const [state, setState] = useState<FileSystemState>({ supported: "showDirectoryPicker" in window, directoryHandle: null, files: [], currentPath: "/", loading: false, error: null });
  const openDirectory = async () => {
    try {
      setState(prev => ({ ...prev, loading: true, error: null }));
      const handle = await (window as any).showDirectoryPicker();
      const files: FsFile[] = [];
      for await (const [name, entry] of (handle as any).entries()) {
        let size: number|undefined;
        if (entry.kind === "file") { try { const f = await entry.getFile(); size = f.size; } catch {} }
        files.push({ name, kind: entry.kind, size });
      }
      setState(prev => ({ ...prev, directoryHandle: handle, files, currentPath: handle.name, loading: false }));
    } catch (e: any) { setState(prev => ({ ...prev, loading: false, error: e?.message || "Cancelled" })); }
  };
  const openFile = async (): Promise<{ name: string; content: string }|null> => {
    try { const [fh] = await (window as any).showOpenFilePicker(); const f = await fh.getFile(); return { name: f.name, content: await f.text() }; } catch { return null; }
  };
  const saveFile = async (content: string, suggestedName = "file.txt"): Promise<boolean> => {
    try { const h = await (window as any).showSaveFilePicker({ suggestedName }); const w = await h.createWritable(); await w.write(content); await w.close(); return true; } catch { return false; }
  };
  return { ...state, openDirectory, openFile, saveFile };
};

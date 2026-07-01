import React, { createContext, useContext, useState, ReactNode } from "react";

export type BootState = "booting" | "login" | "desktop";

export interface WindowState {
  id: string;
  app: string;
  title: string;
  position: { x: number; y: number };
  size: { width: number; height: number };
  isMinimized: boolean;
  isMaximized: boolean;
  zIndex: number;
}

export interface OsContextType {
  bootState: BootState;
  setBootState: (state: BootState) => void;
  windows: WindowState[];
  openWindow: (app: string, title?: string, defaultSize?: { width: number; height: number }) => void;
  closeWindow: (id: string) => void;
  minimizeWindow: (id: string) => void;
  maximizeWindow: (id: string) => void;
  focusWindow: (id: string) => void;
  updateWindowPosition: (id: string, position: { x: number; y: number }) => void;
  updateWindowSize: (id: string, size: { width: number; height: number }) => void;
  focusedWindowId: string | null;
  wallpaper: string;
  setWallpaper: (wallpaper: string) => void;
}

const OsContext = createContext<OsContextType | undefined>(undefined);

export const OsProvider = ({ children }: { children: ReactNode }) => {
  const [bootState, setBootState] = useState<BootState>("booting");
  const [windows, setWindows] = useState<WindowState[]>([]);
  const [focusedWindowId, setFocusedWindowId] = useState<string | null>(null);
  const [wallpaper, setWallpaper] = useState<string>("default");

  const openWindow = (app: string, title?: string, defaultSize = { width: 600, height: 400 }) => {
    const id = `${app}-${Date.now()}`;
    const newWindow: WindowState = {
      id,
      app,
      title: title || app,
      position: { x: 100 + (windows.length * 20), y: 100 + (windows.length * 20) },
      size: defaultSize,
      isMinimized: false,
      isMaximized: false,
      zIndex: windows.length + 1,
    };
    setWindows((prev) => [...prev, newWindow]);
    setFocusedWindowId(id);
  };

  const closeWindow = (id: string) => {
    setWindows((prev) => prev.filter((w) => w.id !== id));
    if (focusedWindowId === id) setFocusedWindowId(null);
  };

  const minimizeWindow = (id: string) => {
    setWindows((prev) => prev.map((w) => (w.id === id ? { ...w, isMinimized: true } : w)));
    if (focusedWindowId === id) setFocusedWindowId(null);
  };

  const maximizeWindow = (id: string) => {
    setWindows((prev) => prev.map((w) => (w.id === id ? { ...w, isMaximized: !w.isMaximized } : w)));
  };

  const focusWindow = (id: string) => {
    setWindows((prev) => {
      const maxZIndex = Math.max(...prev.map((w) => w.zIndex), 0);
      return prev.map((w) => w.id === id ? { ...w, zIndex: maxZIndex + 1, isMinimized: false } : w);
    });
    setFocusedWindowId(id);
  };

  const updateWindowPosition = (id: string, position: { x: number; y: number }) => {
    setWindows((prev) => prev.map((w) => (w.id === id ? { ...w, position } : w)));
  };

  const updateWindowSize = (id: string, size: { width: number; height: number }) => {
    setWindows((prev) => prev.map((w) => (w.id === id ? { ...w, size } : w)));
  };

  return (
    <OsContext.Provider
      value={{
        bootState, setBootState,
        windows, openWindow, closeWindow, minimizeWindow, maximizeWindow,
        focusWindow, updateWindowPosition, updateWindowSize,
        focusedWindowId, wallpaper, setWallpaper,
      }}
    >
      {children}
    </OsContext.Provider>
  );
};

export const useOs = () => {
  const context = useContext(OsContext);
  if (!context) throw new Error("useOs must be used within an OsProvider");
  return context;
};

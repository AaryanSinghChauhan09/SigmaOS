import React, { useState, useEffect } from "react";
import { useOs } from "@/os/OsContext";
import { useDrivers } from "@/os/DriverContext";
import { APP_REGISTRY } from "@/apps/registry";
import { cn } from "@/lib/utils";
import { Battery, BatteryCharging, BatteryLow, Wifi, WifiOff, Volume2, Grid, X, Minus, Square } from "lucide-react";

// Boot screen
const Boot = () => {
  const { setBootState } = useOs();
  useEffect(() => { const t = setTimeout(() => setBootState("login"), 2000); return () => clearTimeout(t); }, []);
  return (
    <div className="fixed inset-0 bg-black flex items-center justify-center">
      <div className="text-center space-y-4">
        <div className="text-6xl font-bold text-primary">Σ</div>
        <div className="text-white text-xl font-light tracking-widest">SigmaOS</div>
        <div className="flex justify-center gap-1 mt-4">
          {[0,1,2].map(i => <div key={i} className="w-2 h-2 rounded-full bg-primary animate-pulse" style={{animationDelay:`${i*200}ms`}}/>)}
        </div>
      </div>
    </div>
  );
};

// Login screen
const Login = () => {
  const { setBootState } = useOs();
  return (
    <div className="fixed inset-0 bg-gradient-to-br from-slate-900 via-violet-950 to-slate-900 flex items-center justify-center">
      <div className="text-center space-y-6">
        <div className="text-7xl font-bold text-primary drop-shadow-[0_0_30px_rgba(139,92,246,0.8)]">Σ</div>
        <div className="text-white text-2xl font-light">SigmaOS</div>
        <p className="text-slate-400 text-sm">Sovereign OS for India</p>
        <button onClick={() => setBootState("desktop")}
          className="px-8 py-3 bg-primary text-white rounded-xl font-medium hover:bg-primary/90 transition-all shadow-lg shadow-primary/30">
          Enter Desktop
        </button>
      </div>
    </div>
  );
};

// Window chrome
const Window = ({ win }: { win: ReturnType<typeof useOs>["windows"][0] }) => {
  const { closeWindow, minimizeWindow, maximizeWindow, focusWindow, updateWindowPosition, updateWindowSize, focusedWindowId } = useOs();
  const appDef = APP_REGISTRY[win.app];
  const Component = appDef?.component;
  const isFocused = win.id === focusedWindowId;

  if (win.isMinimized) return null;

  const style = win.isMaximized
    ? { left:0, top:0, width:"100%", height:"calc(100% - 48px)" }
    : { left:win.position.x, top:win.position.y, width:win.size.width, height:win.size.height };

  return (
    <div
      className={cn("absolute rounded-xl overflow-hidden shadow-2xl border flex flex-col",
        isFocused ? "border-border/80 shadow-black/50" : "border-border/30")}
      style={{ ...style, zIndex: win.zIndex }}
      onMouseDown={() => focusWindow(win.id)}
    >
      {/* Title bar */}
      <div className="flex items-center gap-2 px-3 py-2 bg-card/95 backdrop-blur border-b border-border/50 flex-shrink-0 cursor-default select-none"
        onMouseDown={e => {
          if (win.isMaximized) return;
          const startX = e.clientX - win.position.x, startY = e.clientY - win.position.y;
          const onMove = (me: MouseEvent) => updateWindowPosition(win.id, { x: me.clientX - startX, y: me.clientY - startY });
          const onUp = () => { document.removeEventListener("mousemove", onMove); document.removeEventListener("mouseup", onUp); };
          document.addEventListener("mousemove", onMove); document.addEventListener("mouseup", onUp);
        }}>
        <button onClick={() => closeWindow(win.id)} className="w-3 h-3 rounded-full bg-red-500 hover:bg-red-400 flex-shrink-0"/>
        <button onClick={() => minimizeWindow(win.id)} className="w-3 h-3 rounded-full bg-yellow-500 hover:bg-yellow-400 flex-shrink-0"/>
        <button onClick={() => maximizeWindow(win.id)} className="w-3 h-3 rounded-full bg-green-500 hover:bg-green-400 flex-shrink-0"/>
        <span className="flex-1 text-center text-xs text-muted-foreground font-medium truncate">{win.title}</span>
      </div>
      <div className="flex-1 overflow-hidden bg-background">
        {Component && <Component windowId={win.id} />}
      </div>
    </div>
  );
};

// System tray clock
const Clock = () => {
  const [time, setTime] = useState(new Date());
  useEffect(() => { const t = setInterval(() => setTime(new Date()), 1000); return () => clearInterval(t); }, []);
  return <div className="text-sm font-medium tabular-nums">{time.toLocaleTimeString([], { hour:"2-digit", minute:"2-digit" })}</div>;
};

// Dock
const Dock = () => {
  const { windows, openWindow, focusedWindowId, focusWindow } = useOs();
  const { battery, network } = useDrivers();
  const [hoveredApp, setHoveredApp] = useState<string|null>(null);
  const pinnedApps = ["about", "terminal", "devicemanager"];
  const runningApps = Array.from(new Set(windows.map(w => w.app)));
  const allApps = Array.from(new Set([...pinnedApps, ...runningApps]));

  const BatteryIcon = battery.charging ? BatteryCharging : battery.level < 0.2 ? BatteryLow : Battery;
  const batteryColor = battery.charging ? "text-green-400" : battery.level < 0.2 ? "text-red-400" : "text-foreground/80";

  return (
    <div className="absolute bottom-0 left-0 right-0 h-12 bg-background/60 backdrop-blur-xl border-t border-border/50 flex items-center justify-between px-4">
      <div className="flex items-center gap-1">
        {allApps.map(appId => {
          const def = APP_REGISTRY[appId]; if (!def) return null;
          const Icon = def.icon;
          const isRunning = runningApps.includes(appId);
          const isFocused = windows.some(w => w.app === appId && w.id === focusedWindowId);
          return (
            <div key={appId} className="relative" onMouseEnter={()=>setHoveredApp(appId)} onMouseLeave={()=>setHoveredApp(null)}>
              <button onClick={() => {
                if (isRunning) { const ws = windows.filter(w=>w.app===appId); if(ws.length>0) focusWindow(ws[0].id); }
                else openWindow(appId, def.name, def.defaultSize);
              }} className={cn("w-9 h-9 rounded-lg flex items-center justify-center hover:bg-white/10 transition-all", isFocused&&"bg-white/10")}>
                <Icon size={20} className={isFocused?"text-primary":"text-foreground/80"}/>
              </button>
              {isRunning && <div className="absolute -bottom-0.5 left-1/2 -translate-x-1/2 w-1 h-1 rounded-full bg-primary"/>}
              {hoveredApp===appId && <div className="absolute bottom-10 left-1/2 -translate-x-1/2 px-2 py-1 rounded bg-popover border border-border/50 text-xs whitespace-nowrap">{def.name}</div>}
            </div>
          );
        })}
      </div>
      <div className="flex items-center gap-3 text-foreground/80 text-sm">
        {network.supported?(network.online?<Wifi size={14} className="text-green-400"/>:<WifiOff size={14} className="text-red-400"/>):<Wifi size={14}/>}
        <Volume2 size={14}/>
        <div className="flex items-center gap-1">
          <BatteryIcon size={14} className={batteryColor}/>
          {battery.supported&&<span className={cn("text-xs tabular-nums",batteryColor)}>{Math.round(battery.level*100)}%</span>}
        </div>
        <Clock/>
      </div>
    </div>
  );
};

// Desktop
const DesktopScreen = () => {
  const { windows, openWindow } = useOs();
  const desktopApps = Object.values(APP_REGISTRY);
  return (
    <div className="fixed inset-0 bg-gradient-to-br from-slate-900 via-violet-950 to-slate-900 overflow-hidden">
      {/* Desktop icons */}
      <div className="absolute top-4 left-4 grid grid-cols-1 gap-3">
        {desktopApps.map(def => {
          const Icon = def.icon;
          return (
            <div key={def.id} className="flex flex-col items-center gap-1 cursor-default group"
              onDoubleClick={() => openWindow(def.id, def.name, def.defaultSize)}>
              <div className="w-12 h-12 rounded-xl bg-white/10 group-hover:bg-white/20 backdrop-blur flex items-center justify-center transition-all">
                <Icon size={24} className="text-white"/>
              </div>
              <span className="text-white text-xs text-center max-w-[60px] truncate drop-shadow">{def.name}</span>
            </div>
          );
        })}
      </div>
      {/* Windows */}
      {windows.map(win => <Window key={win.id} win={win}/>)}
      {/* Dock */}
      <Dock/>
    </div>
  );
};

export const OsRoot = () => {
  const { bootState } = useOs();
  return (
    <>
      {bootState === "booting"  && <Boot/>}
      {bootState === "login"    && <Login/>}
      {bootState === "desktop"  && <DesktopScreen/>}
    </>
  );
};

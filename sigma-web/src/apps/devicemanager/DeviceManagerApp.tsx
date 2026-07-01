import React, { useState, useRef, useEffect } from "react";
import { useDrivers } from "@/os/DriverContext";
import { cn } from "@/lib/utils";
import {
  Cpu, Wifi, Battery, MapPin, Activity, Sun, HardDrive, Mic,
  Camera, Monitor, Gamepad2, MessageSquare, Bell, Clipboard,
  Lock, Layers, Usb, Bluetooth, Cable, Mouse, Music, Share2,
  Pen, WifiOff, Volume2, Eye
} from "lucide-react";

type Category =
  | "overview" | "power" | "network" | "location" | "motion"
  | "ambient" | "storage" | "audio" | "camera" | "screencapture"
  | "gamepad" | "speech" | "notifications" | "clipboard"
  | "wakelock" | "screens" | "usb" | "bluetooth" | "serial"
  | "hid" | "midi" | "gpu" | "share" | "pointer";

interface NavItem { id: Category; label: string; icon: React.ElementType; }

const NAV_ITEMS: NavItem[] = [
  { id: "overview",      label: "Overview",      icon: Cpu },
  { id: "power",         label: "Power",         icon: Battery },
  { id: "network",       label: "Network",       icon: Wifi },
  { id: "location",      label: "Location",      icon: MapPin },
  { id: "motion",        label: "Motion",        icon: Activity },
  { id: "ambient",       label: "Ambient Light", icon: Sun },
  { id: "storage",       label: "Storage",       icon: HardDrive },
  { id: "audio",         label: "Audio",         icon: Mic },
  { id: "camera",        label: "Camera",        icon: Camera },
  { id: "screencapture", label: "Screen Capture",icon: Monitor },
  { id: "gamepad",       label: "Gamepads",      icon: Gamepad2 },
  { id: "speech",        label: "Speech",        icon: MessageSquare },
  { id: "notifications", label: "Notifications", icon: Bell },
  { id: "clipboard",     label: "Clipboard",     icon: Clipboard },
  { id: "wakelock",      label: "Wake Lock",     icon: Lock },
  { id: "screens",       label: "Screens",       icon: Layers },
  { id: "usb",           label: "USB",           icon: Usb },
  { id: "bluetooth",     label: "Bluetooth",     icon: Bluetooth },
  { id: "serial",        label: "Serial",        icon: Cable },
  { id: "hid",           label: "HID Devices",   icon: Mouse },
  { id: "midi",          label: "MIDI",          icon: Music },
  { id: "gpu",           label: "GPU",           icon: Eye },
  { id: "share",         label: "Share",         icon: Share2 },
  { id: "pointer",       label: "Pointer",       icon: Pen },
];

type Status = "active" | "unsupported" | "denied" | "idle" | "requesting";

const Badge = ({ status }: { status: Status }) => {
  const map: Record<Status, string> = {
    active:      "bg-green-500/20 text-green-400 border-green-500/30",
    unsupported: "bg-zinc-500/20 text-zinc-400 border-zinc-500/30",
    denied:      "bg-red-500/20 text-red-400 border-red-500/30",
    idle:        "bg-blue-500/20 text-blue-400 border-blue-500/30",
    requesting:  "bg-yellow-500/20 text-yellow-400 border-yellow-500/30",
  };
  const labels: Record<Status, string> = {
    active:"Active", unsupported:"Unsupported", denied:"Denied",
    idle:"Available", requesting:"Requesting…"
  };
  return <span className={cn("px-2 py-0.5 rounded-full text-xs font-medium border", map[status])}>{labels[status]}</span>;
};

const StatusDot = ({ status }: { status: "active"|"unsupported"|"denied"|"idle" }) => {
  const map = { active:"bg-green-400", unsupported:"bg-zinc-500", denied:"bg-red-400", idle:"bg-blue-400" };
  return <span className={cn("w-2 h-2 rounded-full flex-shrink-0", map[status])} />;
};

const ST = ({ children }: { children: React.ReactNode }) =>
  <h3 className="text-xs font-semibold uppercase tracking-widest text-muted-foreground mb-3">{children}</h3>;

const Metric = ({ label, value }: { label: string; value: React.ReactNode }) => (
  <div className="flex flex-col gap-0.5">
    <span className="text-xs text-muted-foreground">{label}</span>
    <span className="text-sm font-medium text-foreground">{value}</span>
  </div>
);

const Btn = ({ onClick, children, variant="default", disabled, testId }: {
  onClick: () => void; children: React.ReactNode;
  variant?: "default"|"danger"|"success"; disabled?: boolean; testId?: string;
}) => {
  const v = {
    default: "bg-primary/20 hover:bg-primary/30 text-primary border-primary/30",
    danger:  "bg-red-500/20 hover:bg-red-500/30 text-red-400 border-red-500/30",
    success: "bg-green-500/20 hover:bg-green-500/30 text-green-400 border-green-500/30",
  };
  return (
    <button onClick={onClick} disabled={disabled} data-testid={testId}
      className={cn("px-3 py-1.5 rounded-lg text-xs font-medium border transition-all", v[variant], disabled && "opacity-40 cursor-not-allowed")}>
      {children}
    </button>
  );
};

const Card = ({ children, className }: { children: React.ReactNode; className?: string }) =>
  <div className={cn("bg-card/50 border border-border/50 rounded-xl p-4", className)}>{children}</div>;

function getDriverStatus(cat: Category, d: ReturnType<typeof useDrivers>): "active"|"unsupported"|"denied"|"idle" {
  switch (cat) {
    case "power":         return d.battery.supported ? "active" : "unsupported";
    case "network":       return d.network.supported ? (d.network.online ? "active" : "denied") : "unsupported";
    case "location":      return !d.geolocation.supported ? "unsupported" : d.geolocation.permission === "granted" ? "active" : d.geolocation.permission === "denied" ? "denied" : "idle";
    case "motion":        return !d.motion.supported ? "unsupported" : d.motion.permission === "granted" ? "active" : d.motion.permission === "denied" ? "denied" : "idle";
    case "ambient":       return d.ambientLight.supported ? "active" : "idle";
    case "storage":       return d.opfs.supported ? "active" : "unsupported";
    case "audio":         return d.audio.permission === "granted" ? "active" : d.audio.permission === "denied" ? "denied" : d.audio.supported ? "idle" : "unsupported";
    case "camera":        return d.camera.permission === "granted" ? "active" : d.camera.permission === "denied" ? "denied" : d.camera.supported ? "idle" : "unsupported";
    case "screencapture": return d.screenCapture.stream ? "active" : d.screenCapture.supported ? "idle" : "unsupported";
    case "gamepad":       return d.gamepad.gamepads.length > 0 ? "active" : d.gamepad.supported ? "idle" : "unsupported";
    case "speech":        return d.speech.listening ? "active" : d.speech.supported ? "idle" : "unsupported";
    case "notifications": return d.notification.permission === "granted" ? "active" : d.notification.permission === "denied" ? "denied" : d.notification.supported ? "idle" : "unsupported";
    case "clipboard":     return d.clipboard.supported ? "active" : "unsupported";
    case "wakelock":      return d.wakeLock.active ? "active" : d.wakeLock.supported ? "idle" : "unsupported";
    case "screens":       return d.multiScreen.permission === "granted" ? "active" : "idle";
    case "usb":           return d.usb.devices.length > 0 ? "active" : d.usb.supported ? "idle" : "unsupported";
    case "bluetooth":     return d.bluetooth.device?.connected ? "active" : d.bluetooth.supported ? "idle" : "unsupported";
    case "serial":        return d.serial.connected ? "active" : d.serial.supported ? "idle" : "unsupported";
    case "hid":           return d.hid.devices.length > 0 ? "active" : d.hid.supported ? "idle" : "unsupported";
    case "midi":          return d.midi.permission === "granted" ? "active" : d.midi.supported ? "idle" : "unsupported";
    case "gpu":           return d.gpu.adapter ? "active" : d.gpu.supported ? "idle" : "unsupported";
    case "share":         return d.share.supported ? "active" : "unsupported";
    case "pointer":       return "active";
    default:              return "active";
  }
}

export const DeviceManagerApp = ({ windowId }: { windowId: string }) => {
  const [active, setActive] = useState<Category>("overview");
  const drivers = useDrivers();
  return (
    <div className="flex h-full bg-background text-foreground overflow-hidden" data-testid="device-manager">
      <div className="w-52 flex-shrink-0 border-r border-border/50 bg-sidebar overflow-y-auto py-2">
        {NAV_ITEMS.map(item => {
          const Icon = item.icon;
          const status = getDriverStatus(item.id, drivers);
          return (
            <button key={item.id} onClick={() => setActive(item.id)} data-testid={`nav-${item.id}`}
              className={cn("w-full flex items-center gap-3 px-3 py-2 text-sm transition-all",
                active === item.id ? "bg-primary/15 text-primary" : "text-sidebar-foreground hover:bg-sidebar-accent hover:text-foreground")}>
              <Icon size={14} className="flex-shrink-0" />
              <span className="flex-1 text-left truncate">{item.label}</span>
              <StatusDot status={status} />
            </button>
          );
        })}
      </div>
      <div className="flex-1 overflow-y-auto p-5">
        <Panel category={active} drivers={drivers} />
      </div>
    </div>
  );
};

const Panel = ({ category, drivers: d }: { category: Category; drivers: ReturnType<typeof useDrivers> }) => {
  const [opfsFile, setOpfsFile] = useState("test.txt");
  const [opfsContent, setOpfsContent] = useState("Hello from SigmaOS OPFS!");
  const [opfsRead, setOpfsRead] = useState<string|null>(null);
  const [notifTitle, setNotifTitle] = useState("SigmaOS Alert");
  const [notifBody, setNotifBody] = useState("Driver test notification");
  const [copyText, setCopyText] = useState("Copy me to clipboard!");
  const [speakText, setSpeakText] = useState("Hello from SigmaOS speech driver");
  const [serialSend, setSerialSend] = useState("AT\r\n");
  const [midiNote, setMidiNote] = useState(60);
  const videoRef = useRef<HTMLVideoElement>(null);
  const [snapshot, setSnapshot] = useState<string|null>(null);
  useEffect(() => { if (videoRef.current && d.camera.stream) videoRef.current.srcObject = d.camera.stream; }, [d.camera.stream]);

  switch (category) {
    case "overview": return (
      <div>
        <h2 className="text-lg font-semibold mb-1">Hardware Overview</h2>
        <p className="text-sm text-muted-foreground mb-5">All system drivers and their support status.</p>
        <div className="grid grid-cols-2 gap-3">
          {NAV_ITEMS.filter(n => n.id !== "overview").map(item => {
            const Icon = item.icon; const status = getDriverStatus(item.id, d);
            return (<Card key={item.id} className="flex items-center gap-3">
              <div className="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center flex-shrink-0"><Icon size={16} className="text-primary" /></div>
              <div className="flex-1 min-w-0"><p className="text-xs font-medium truncate">{item.label}</p></div>
              <Badge status={status} /></Card>);
          })}
        </div>
      </div>
    );

    case "power": return (
      <div><h2 className="text-lg font-semibold mb-1">Power</h2><p className="text-sm text-muted-foreground mb-5">Battery Status API</p>
        {!d.battery.supported ? <Card><p className="text-sm text-muted-foreground">Battery Status API not supported.</p></Card> : (
          <div className="space-y-4">
            <Card className="flex items-center gap-6">
              <div className="relative w-24 h-14 border-2 border-border rounded-lg flex items-center justify-end pr-1">
                <div className={cn("h-10 rounded-md transition-all", d.battery.charging ? "bg-green-500" : d.battery.level < 0.2 ? "bg-red-500" : "bg-primary")} style={{ width: `${d.battery.level * 100}%`, maxWidth: "calc(100% - 4px)" }} />
                <div className="absolute -right-2 top-1/2 -translate-y-1/2 w-2 h-4 bg-border rounded-r" />
              </div>
              <div className="space-y-1"><p className="text-3xl font-bold tabular-nums">{Math.round(d.battery.level * 100)}%</p><p className="text-sm text-muted-foreground">{d.battery.charging ? "Charging" : "Discharging"}</p></div>
            </Card>
            <div className="grid grid-cols-2 gap-3">
              <Card><Metric label="Charging Time" value={d.battery.chargingTime === Infinity ? "—" : `${Math.round(d.battery.chargingTime / 60)} min`} /></Card>
              <Card><Metric label="Time Remaining" value={d.battery.dischargingTime === Infinity ? "—" : `${Math.round(d.battery.dischargingTime / 60)} min`} /></Card>
            </div>
          </div>
        )}
      </div>
    );
    case "network": return (
      <div><h2 className="text-lg font-semibold mb-1">Network</h2><p className="text-sm text-muted-foreground mb-5">Network Information API</p>
        <div className="space-y-4">
          <Card className="flex items-center gap-4">
            {d.network.online ? <Wifi size={32} className="text-green-400" /> : <WifiOff size={32} className="text-red-400" />}
            <div><p className="text-lg font-semibold">{d.network.online ? "Connected" : "Offline"}</p><p className="text-sm text-muted-foreground">{d.network.effectiveType.toUpperCase()} connection</p></div>
            <Badge status={d.network.online ? "active" : "denied"} />
          </Card>
          <div className="grid grid-cols-3 gap-3">
            <Card><Metric label="Download Speed" value={d.network.supported ? `${d.network.downlink} Mbps` : "—"} /></Card>
            <Card><Metric label="Round-trip Time" value={d.network.supported ? `${d.network.rtt} ms` : "—"} /></Card>
            <Card><Metric label="Data Saver" value={d.network.saveData ? "On" : "Off"} /></Card>
          </div>
        </div>
      </div>
    );

    case "storage": return (
      <div><h2 className="text-lg font-semibold mb-1">Storage</h2><p className="text-sm text-muted-foreground mb-5">OPFS + File System Access API</p>
        <div className="space-y-4">
          {d.opfs.quota !== null && <Card><ST>OPFS Quota</ST>
            <div className="flex justify-between text-xs mb-1"><span>{((d.opfs.usage??0)/1024/1024).toFixed(2)} MB used</span><span>{(d.opfs.quota/1024/1024).toFixed(0)} MB total</span></div>
            <div className="h-2 bg-border rounded-full overflow-hidden"><div className="h-full bg-primary rounded-full" style={{width:`${((d.opfs.usage??0)/d.opfs.quota)*100}%`}} /></div></Card>}
          <Card><ST>Write File</ST>
            <input value={opfsFile} onChange={e=>setOpfsFile(e.target.value)} placeholder="filename" className="w-full bg-input border border-border rounded-lg px-2 py-1 text-xs mb-2" />
            <textarea value={opfsContent} onChange={e=>setOpfsContent(e.target.value)} className="w-full bg-input border border-border rounded-lg px-2 py-1 text-xs font-mono h-16 mb-2 resize-none" />
            <div className="flex gap-2">
              <Btn onClick={async()=>{ await d.opfs.writeFile(opfsFile, opfsContent); }}>Write</Btn>
              <Btn onClick={async()=>{ const c=await d.opfs.readFile(opfsFile); setOpfsRead(c); }}>Read</Btn>
              <Btn onClick={async()=>{ await d.opfs.deleteFile(opfsFile); }} variant="danger">Delete</Btn>
            </div>
            {opfsRead!==null && <pre className="mt-2 text-xs font-mono bg-muted rounded-lg p-2 overflow-auto max-h-24">{opfsRead}</pre>}
          </Card>
          <Card><ST>OPFS Files ({d.opfs.files.length})</ST>
            {d.opfs.files.length===0 ? <p className="text-xs text-muted-foreground">No files yet.</p> :
              <div className="space-y-1">{d.opfs.files.map(f=><div key={f} className="flex items-center gap-2 text-xs py-1 border-b border-border/30 last:border-0"><HardDrive size={12} className="text-muted-foreground"/><span className="font-mono">{f}</span></div>)}</div>}
          </Card>
          <Card><ST>Real File System Access</ST>
            <div className="flex gap-2"><Btn onClick={d.filesystem.openDirectory}>Open Directory</Btn><Btn onClick={async()=>{await d.filesystem.openFile();}}>Open File</Btn></div>
            {d.filesystem.currentPath!=="/" && <p className="text-xs text-muted-foreground mt-2">Open: {d.filesystem.currentPath} ({d.filesystem.files.length} items)</p>}
          </Card>
        </div>
      </div>
    );

    case "audio": return (
      <div><h2 className="text-lg font-semibold mb-1">Audio</h2><p className="text-sm text-muted-foreground mb-5">Web Audio API</p>
        <div className="space-y-4">
          <Card className="flex items-center justify-between">
            <div><p className="text-sm font-medium">Microphone</p><Badge status={d.audio.permission==="granted"?"active":d.audio.permission==="denied"?"denied":"idle"}/></div>
            <div className="flex gap-2">
              {d.audio.permission!=="granted"?<Btn onClick={d.audio.requestMicrophone}>Enable Mic</Btn>:<Btn onClick={d.audio.stopMicrophone} variant="danger">Stop Mic</Btn>}
              <Btn onClick={()=>d.audio.playBeep(440,0.3)}>Beep A4</Btn>
            </div>
          </Card>
          {d.audio.permission==="granted"&&d.audio.frequencyData&&(<Card>
            <ST>Frequency Spectrum</ST>
            <div className="flex items-end gap-0.5 h-20">{Array.from(d.audio.frequencyData).filter((_,i)=>i%2===0).map((v,i)=><div key={i} className="flex-1 bg-primary rounded-t" style={{height:`${(v/255)*100}%`}}/>)}</div>
            <div className="mt-2 flex items-center gap-2"><Volume2 size={12} className="text-muted-foreground"/><div className="flex-1 h-1.5 bg-border rounded-full"><div className="h-full bg-green-400 rounded-full" style={{width:`${d.audio.volume*100}%`}}/></div><span className="text-xs">{Math.round(d.audio.volume*100)}%</span></div>
          </Card>)}
        </div>
      </div>
    );
    case "camera": return (
      <div><h2 className="text-lg font-semibold mb-1">Camera</h2><p className="text-sm text-muted-foreground mb-5">getUserMedia video</p>
        <div className="space-y-4">
          <Card className="flex items-center justify-between">
            <Badge status={d.camera.permission==="granted"?"active":d.camera.permission==="denied"?"denied":"idle"}/>
            <div className="flex gap-2">
              {d.camera.permission!=="granted"?<Btn onClick={()=>d.camera.requestCamera()}>Start Camera</Btn>:<><Btn onClick={d.camera.switchCamera}>Flip</Btn><Btn onClick={()=>{const s=d.camera.takeSnapshot(videoRef);if(s)setSnapshot(s);}}>Snapshot</Btn><Btn onClick={d.camera.stopCamera} variant="danger">Stop</Btn></>}
            </div>
          </Card>
          {d.camera.stream&&<Card><video ref={videoRef} autoPlay muted playsInline className="w-full rounded-lg bg-black"/></Card>}
          {snapshot&&<Card><ST>Snapshot</ST><img src={snapshot} className="w-full rounded-lg"/></Card>}
        </div>
      </div>
    );
    case "screencapture": return (
      <div><h2 className="text-lg font-semibold mb-1">Screen Capture</h2><p className="text-sm text-muted-foreground mb-5">getDisplayMedia API</p>
        <div className="space-y-4">
          <Card className="flex items-center justify-between flex-wrap gap-2">
            <Badge status={d.screenCapture.stream?"active":"idle"}/>
            <div className="flex gap-2 flex-wrap">
              {!d.screenCapture.stream?<Btn onClick={d.screenCapture.startCapture}>Start Capture</Btn>:<>
                {!d.screenCapture.recording?<Btn onClick={d.screenCapture.startRecording} variant="success">Record</Btn>:<Btn onClick={d.screenCapture.stopRecording} variant="danger">Stop Rec</Btn>}
                <Btn onClick={d.screenCapture.stopCapture} variant="danger">Stop Capture</Btn>
              </>}
              {d.screenCapture.recordedChunks.length>0&&<Btn onClick={d.screenCapture.downloadRecording} variant="success">Download</Btn>}
            </div>
          </Card>
          {d.screenCapture.recording&&<Card className="border-red-500/30 bg-red-500/5"><div className="flex items-center gap-2 text-red-400 text-sm"><div className="w-2 h-2 rounded-full bg-red-400 animate-pulse"/>Recording…</div></Card>}
        </div>
      </div>
    );

    case "gamepad": return (
      <div><h2 className="text-lg font-semibold mb-1">Gamepads</h2><p className="text-sm text-muted-foreground mb-5">Gamepad API</p>
        {d.gamepad.gamepads.length===0?<Card><p className="text-sm text-muted-foreground">No gamepads. Connect a controller and press a button.</p></Card>:
          <div className="space-y-4">{d.gamepad.gamepads.map(gp=><Card key={gp.index}>
            <div className="flex items-center justify-between mb-3"><p className="text-sm font-medium truncate max-w-xs">{gp.id}</p><Badge status={gp.connected?"active":"denied"}/></div>
            <ST>Buttons ({gp.buttons.length})</ST>
            <div className="flex flex-wrap gap-1.5 mb-3">{gp.buttons.map((btn,i)=><div key={i} className={cn("w-7 h-7 rounded-md flex items-center justify-center text-xs font-mono border",btn.pressed?"bg-primary text-white border-primary":"bg-muted border-border text-muted-foreground")}>{i}</div>)}</div>
            <ST>Axes ({gp.axes.length})</ST>
            <div className="space-y-1">{gp.axes.map((val,i)=><div key={i} className="flex items-center gap-2"><span className="text-xs text-muted-foreground w-8">A{i}</span><div className="flex-1 h-1.5 bg-border rounded-full relative"><div className="absolute h-full w-1.5 bg-primary rounded-full" style={{left:`${((val+1)/2)*100}%`,transform:"translateX(-50%)"}}/></div><span className="text-xs font-mono w-10 text-right">{val.toFixed(2)}</span></div>)}</div>
          </Card>)}</div>}
      </div>
    );
    case "speech": return (
      <div><h2 className="text-lg font-semibold mb-1">Speech</h2><p className="text-sm text-muted-foreground mb-5">SpeechRecognition + SpeechSynthesis</p>
        <div className="space-y-4">
          <Card className="flex items-center justify-between">
            <div><p className="text-sm font-medium">Speech Recognition</p><Badge status={d.speech.listening?"active":d.speech.supported?"idle":"unsupported"}/></div>
            <div className="flex gap-2">{!d.speech.listening?<Btn onClick={d.speech.startListening} disabled={!d.speech.supported}>Listen</Btn>:<Btn onClick={d.speech.stopListening} variant="danger">Stop</Btn>}<Btn onClick={d.speech.clearTranscript}>Clear</Btn></div>
          </Card>
          {(d.speech.transcript||d.speech.finalTranscript)&&<Card><ST>Transcript</ST><p className="text-sm font-mono">{d.speech.finalTranscript}<span className="text-muted-foreground">{d.speech.transcript}</span></p></Card>}
          <Card><ST>Text-to-Speech</ST>
            <textarea value={speakText} onChange={e=>setSpeakText(e.target.value)} className="w-full bg-input border border-border rounded-lg px-2 py-1 text-xs font-mono h-16 mb-2 resize-none"/>
            <Btn onClick={()=>d.speech.speak(speakText)} disabled={!d.speech.ttsSupported}>Speak</Btn>
          </Card>
        </div>
      </div>
    );
    case "notifications": return (
      <div><h2 className="text-lg font-semibold mb-1">Notifications</h2><p className="text-sm text-muted-foreground mb-5">Notifications API</p>
        <div className="space-y-4">
          <Card className="flex items-center justify-between"><div><p className="text-sm font-medium">Permission: {d.notification.permission}</p><Badge status={d.notification.permission==="granted"?"active":d.notification.permission==="denied"?"denied":"idle"}/></div>{d.notification.permission!=="granted"&&<Btn onClick={d.notification.requestPermission}>Request</Btn>}</Card>
          <Card><ST>Send Notification</ST>
            <input value={notifTitle} onChange={e=>setNotifTitle(e.target.value)} placeholder="Title" className="w-full bg-input border border-border rounded-lg px-2 py-1 text-xs mb-2"/>
            <input value={notifBody} onChange={e=>setNotifBody(e.target.value)} placeholder="Body" className="w-full bg-input border border-border rounded-lg px-2 py-1 text-xs mb-2"/>
            <Btn onClick={()=>d.notification.sendNotification(notifTitle,{body:notifBody})} disabled={d.notification.permission!=="granted"}>Send</Btn>
          </Card>
        </div>
      </div>
    );
    case "clipboard": return (
      <div><h2 className="text-lg font-semibold mb-1">Clipboard</h2><p className="text-sm text-muted-foreground mb-5">Clipboard API</p>
        <div className="space-y-4">
          <Card><ST>Write</ST><textarea value={copyText} onChange={e=>setCopyText(e.target.value)} className="w-full bg-input border border-border rounded-lg px-2 py-1 text-xs font-mono h-16 mb-2 resize-none"/><Btn onClick={()=>d.clipboard.copy(copyText)}>Copy</Btn>{d.clipboard.lastCopied&&<p className="text-xs text-green-400 mt-1">Copied!</p>}</Card>
          <Card><ST>Read</ST><Btn onClick={()=>d.clipboard.paste()}>Paste</Btn>{d.clipboard.lastRead&&<pre className="mt-2 text-xs font-mono bg-muted rounded-lg p-2 overflow-auto max-h-24">{d.clipboard.lastRead}</pre>}</Card>
        </div>
      </div>
    );
    case "wakelock": return (
      <div><h2 className="text-lg font-semibold mb-1">Wake Lock</h2><p className="text-sm text-muted-foreground mb-5">Screen Wake Lock API</p>
        <Card className="flex items-center justify-between">
          <div><p className="text-sm font-medium">{d.wakeLock.active?"Wake lock active":"Screen can sleep"}</p><Badge status={d.wakeLock.active?"active":d.wakeLock.supported?"idle":"unsupported"}/></div>
          {d.wakeLock.supported&&(d.wakeLock.active?<Btn onClick={d.wakeLock.release} variant="danger">Release</Btn>:<Btn onClick={d.wakeLock.acquire}>Acquire</Btn>)}
        </Card>
      </div>
    );
    case "screens": return (
      <div><h2 className="text-lg font-semibold mb-1">Screens</h2><p className="text-sm text-muted-foreground mb-5">Window Management API</p>
        <div className="space-y-4">
          <Card className="flex items-center justify-between"><div><p className="text-sm font-medium">Multi-Screen</p><Badge status={d.multiScreen.permission==="granted"?"active":"idle"}/></div>{d.multiScreen.supported&&d.multiScreen.permission!=="granted"&&<Btn onClick={d.multiScreen.requestScreenDetails}>Request</Btn>}</Card>
          {d.multiScreen.screens.map(s=><Card key={s.id}><div className="flex items-center justify-between mb-2"><p className="text-sm font-medium">{s.label}</p>{s.isPrimary&&<Badge status="active"/>}</div><div className="grid grid-cols-3 gap-3"><Metric label="Resolution" value={`${s.width}×${s.height}`}/><Metric label="Position" value={`(${s.left},${s.top})`}/><Metric label="DPR" value={s.devicePixelRatio+"x"}/></div></Card>)}
        </div>
      </div>
    );
    case "usb": return (
      <div><h2 className="text-lg font-semibold mb-1">USB</h2><p className="text-sm text-muted-foreground mb-5">WebUSB API</p>
        <div className="space-y-4">
          <Card className="flex items-center justify-between"><Badge status={d.usb.devices.length>0?"active":d.usb.supported?"idle":"unsupported"}/><div className="flex gap-2"><Btn onClick={d.usb.requestDevice} disabled={!d.usb.supported}>Pair Device</Btn><Btn onClick={d.usb.refresh}><span className="text-xs">↻</span></Btn></div></Card>
          {d.usb.devices.map((dev,i)=><Card key={i}><p className="text-sm font-medium">{dev.productName||"Unknown Device"}</p><p className="text-xs text-muted-foreground">{dev.manufacturerName}</p><div className="grid grid-cols-2 gap-2 mt-2"><Metric label="Vendor ID" value={`0x${dev.vendorId.toString(16).padStart(4,"0")}`}/><Metric label="Product ID" value={`0x${dev.productId.toString(16).padStart(4,"0")}`}/></div></Card>)}
        </div>
      </div>
    );
    case "bluetooth": return (
      <div><h2 className="text-lg font-semibold mb-1">Bluetooth</h2><p className="text-sm text-muted-foreground mb-5">Web Bluetooth API</p>
        <div className="space-y-4">
          <Card className="flex items-center justify-between"><Badge status={d.bluetooth.device?.connected?"active":d.bluetooth.supported?"idle":"unsupported"}/><div className="flex gap-2"><Btn onClick={d.bluetooth.requestDevice} disabled={!d.bluetooth.supported||d.bluetooth.scanning}>{d.bluetooth.scanning?"Scanning…":"Scan"}</Btn>{d.bluetooth.device&&<Btn onClick={d.bluetooth.disconnect} variant="danger">Disconnect</Btn>}</div></Card>
          {d.bluetooth.device&&<Card><p className="text-sm font-medium">{d.bluetooth.device.name||"Unknown Device"}</p><p className="text-xs text-muted-foreground font-mono">{d.bluetooth.device.id}</p><Badge status={d.bluetooth.device.connected?"active":"denied"}/></Card>}
        </div>
      </div>
    );
    case "serial": return (
      <div><h2 className="text-lg font-semibold mb-1">Serial</h2><p className="text-sm text-muted-foreground mb-5">Web Serial API</p>
        <div className="space-y-4">
          <Card className="flex items-center justify-between"><Badge status={d.serial.connected?"active":d.serial.supported?"idle":"unsupported"}/>{!d.serial.connected?<Btn onClick={d.serial.requestPort} disabled={!d.serial.supported}>Connect</Btn>:<Btn onClick={d.serial.disconnect} variant="danger">Disconnect</Btn>}</Card>
          {d.serial.connected&&<Card><ST>Send</ST><div className="flex gap-2 mb-2"><input value={serialSend} onChange={e=>setSerialSend(e.target.value)} className="flex-1 bg-input border border-border rounded-lg px-2 py-1 text-xs font-mono"/><Btn onClick={()=>d.serial.send(serialSend)}>Send</Btn></div><ST>Received</ST><pre className="text-xs font-mono bg-muted rounded-lg p-2 h-24 overflow-auto">{d.serial.receivedData||"—"}</pre></Card>}
        </div>
      </div>
    );
    case "hid": return (
      <div><h2 className="text-lg font-semibold mb-1">HID Devices</h2><p className="text-sm text-muted-foreground mb-5">WebHID API</p>
        <div className="space-y-4">
          <Card className="flex items-center justify-between"><Badge status={d.hid.devices.length>0?"active":d.hid.supported?"idle":"unsupported"}/><Btn onClick={d.hid.requestDevice} disabled={!d.hid.supported}>Pair</Btn></Card>
          {d.hid.devices.map((dev,i)=><Card key={i} className="flex items-center justify-between"><div><p className="text-sm font-medium">{dev.productName||"Unknown HID"}</p><p className="text-xs text-muted-foreground font-mono">0x{dev.vendorId.toString(16)}:0x{dev.productId.toString(16)}</p></div>{!dev.opened&&<Btn onClick={()=>d.hid.openDevice(dev.vendorId,dev.productId)}>Open</Btn>}{dev.opened&&<Badge status="active"/>}</Card>)}
        </div>
      </div>
    );
    case "midi": return (
      <div><h2 className="text-lg font-semibold mb-1">MIDI</h2><p className="text-sm text-muted-foreground mb-5">Web MIDI API</p>
        <div className="space-y-4">
          <Card className="flex items-center justify-between"><Badge status={d.midi.permission==="granted"?"active":d.midi.supported?"idle":"unsupported"}/>{d.midi.permission!=="granted"&&<Btn onClick={d.midi.requestAccess} disabled={!d.midi.supported}>Request</Btn>}</Card>
          {d.midi.permission==="granted"&&<><Card><ST>Inputs ({d.midi.inputs.length})</ST>{d.midi.inputs.map(inp=><div key={inp.id} className="text-xs py-1 border-b border-border/30 last:border-0"><span className="font-medium">{inp.name}</span><span className="text-muted-foreground ml-2">{inp.manufacturer}</span></div>)}</Card>
          <Card><ST>Test Note</ST><div className="flex items-center gap-3 mb-2"><label className="text-xs">Note: {midiNote}</label><input type="range" min="21" max="108" value={midiNote} onChange={e=>setMidiNote(+e.target.value)} className="flex-1"/></div>{d.midi.outputs.map(out=><Btn key={out.id} onClick={()=>d.midi.sendNote(out.id,midiNote)}>Play on {out.name}</Btn>)}</Card></>}
        </div>
      </div>
    );
    case "gpu": return (
      <div><h2 className="text-lg font-semibold mb-1">GPU</h2><p className="text-sm text-muted-foreground mb-5">WebGPU API</p>
        <div className="space-y-4">
          {d.gpu.error&&<Card className="border-red-500/30"><p className="text-xs text-red-400">{d.gpu.error}</p></Card>}
          {d.gpu.adapter&&<><Card><ST>Adapter Info</ST><div className="grid grid-cols-2 gap-3"><Metric label="Vendor" value={d.gpu.adapter.vendor}/><Metric label="Architecture" value={d.gpu.adapter.architecture}/><Metric label="Type" value={d.gpu.adapter.deviceType}/></div></Card>
          {d.gpu.features.length>0&&<Card><ST>Features ({d.gpu.features.length})</ST><div className="flex flex-wrap gap-1">{d.gpu.features.map(f=><span key={f} className="px-2 py-0.5 rounded bg-primary/10 text-primary text-xs font-mono">{f}</span>)}</div></Card>}</>}
          {!d.gpu.adapter&&!d.gpu.error&&<Card><p className="text-sm text-muted-foreground">{d.gpu.supported?"Loading GPU…":"WebGPU not supported."}</p></Card>}
        </div>
      </div>
    );
    case "share": return (
      <div><h2 className="text-lg font-semibold mb-1">Share</h2><p className="text-sm text-muted-foreground mb-5">Web Share API</p>
        <Card className="flex items-center justify-between"><Badge status={d.share.supported?"active":"unsupported"}/><Btn onClick={()=>d.share.share({title:"SigmaOS",text:"India's sovereign OS",url:window.location.href})} disabled={!d.share.supported}>Share SigmaOS</Btn></Card>
      </div>
    );
    case "pointer": return (
      <div><h2 className="text-lg font-semibold mb-1">Pointer</h2><p className="text-sm text-muted-foreground mb-5">Pointer Events API</p>
        <div className="space-y-4">
          <div className="grid grid-cols-3 gap-3"><Card><Metric label="Pen Detected" value={d.pointer.hasPen?"Yes":"No"}/></Card><Card><Metric label="Touch Detected" value={d.pointer.hasTouch?"Yes":"No"}/></Card><Card><Metric label="Active" value={d.pointer.activePointers.length}/></Card></div>
          {d.pointer.activePointers.map(ptr=><Card key={ptr.pointerId}><div className="flex items-center justify-between mb-2"><p className="text-sm font-medium capitalize">{ptr.pointerType}</p><span className="text-xs text-muted-foreground">ID: {ptr.pointerId}</span></div><div className="grid grid-cols-3 gap-2"><Metric label="X" value={ptr.x.toFixed(0)}/><Metric label="Y" value={ptr.y.toFixed(0)}/><Metric label="Pressure" value={ptr.pressure.toFixed(2)}/></div></Card>)}
          {d.pointer.activePointers.length===0&&<Card><p className="text-sm text-muted-foreground">Interact with this panel to see pointer data.</p></Card>}
        </div>
      </div>
    );
    default: return <div><p className="text-muted-foreground">Select a driver from the sidebar.</p></div>;
  }
};

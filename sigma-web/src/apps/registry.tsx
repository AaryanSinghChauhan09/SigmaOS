import React from "react";
import { Terminal, Folder, Settings, Info, Calculator, Globe, Image as ImageIcon, PenTool, Cpu } from "lucide-react";
import { DeviceManagerApp } from "./devicemanager/DeviceManagerApp";

// Stub app components for apps not yet ported to this structure
const StubApp = ({ title }: { title: string }) => (
  <div className="flex h-full items-center justify-center text-muted-foreground">
    <p>{title} — coming soon</p>
  </div>
);

const AboutApp = ({ windowId }: { windowId: string }) => (
  <div className="p-8 space-y-4">
    <div className="text-4xl font-bold text-primary">Σ SigmaOS</div>
    <p className="text-muted-foreground">The sovereign OS built for 1.4 billion Indians.</p>
    <div className="space-y-1 text-sm">
      <p><span className="text-muted-foreground">Version:</span> 0.1.0-dev</p>
      <p><span className="text-muted-foreground">Profile:</span> Browser Demo</p>
      <p><span className="text-muted-foreground">Drivers:</span> 24 Web API drivers</p>
      <p><span className="text-muted-foreground">License:</span> GPL-2.0-only</p>
    </div>
  </div>
);

const TerminalApp = ({ windowId }: { windowId: string }) => <StubApp title="Terminal" />;
const FilesApp = ({ windowId }: { windowId: string }) => <StubApp title="Files" />;
const BrowserApp = ({ windowId }: { windowId: string }) => <StubApp title="Sigma Web" />;
const EditorApp = ({ windowId }: { windowId: string }) => <StubApp title="Sigma Edit" />;
const ViewerApp = ({ windowId }: { windowId: string }) => <StubApp title="Gallery" />;
const SettingsApp = ({ windowId }: { windowId: string }) => <StubApp title="Settings" />;
const CalculatorApp = ({ windowId }: { windowId: string }) => <StubApp title="Calculator" />;

export interface AppDefinition {
  id: string;
  name: string;
  icon: React.ElementType;
  component: React.ComponentType<{ windowId: string }>;
  defaultSize?: { width: number; height: number };
}

export const APP_REGISTRY: Record<string, AppDefinition> = {
  about:         { id: "about",         name: "About SigmaOS",  icon: Info,       component: AboutApp,         defaultSize: { width: 450,  height: 500 } },
  terminal:      { id: "terminal",      name: "Terminal",       icon: Terminal,   component: TerminalApp,      defaultSize: { width: 700,  height: 450 } },
  files:         { id: "files",         name: "Files",          icon: Folder,     component: FilesApp,         defaultSize: { width: 800,  height: 500 } },
  browser:       { id: "browser",       name: "Sigma Web",      icon: Globe,      component: BrowserApp,       defaultSize: { width: 1000, height: 600 } },
  editor:        { id: "editor",        name: "Sigma Edit",     icon: PenTool,    component: EditorApp,        defaultSize: { width: 800,  height: 600 } },
  viewer:        { id: "viewer",        name: "Gallery",        icon: ImageIcon,  component: ViewerApp,        defaultSize: { width: 800,  height: 600 } },
  settings:      { id: "settings",      name: "Settings",       icon: Settings,   component: SettingsApp,      defaultSize: { width: 700,  height: 500 } },
  calculator:    { id: "calculator",    name: "Calculator",     icon: Calculator, component: CalculatorApp,    defaultSize: { width: 320,  height: 450 } },
  devicemanager: { id: "devicemanager", name: "Device Manager", icon: Cpu,        component: DeviceManagerApp, defaultSize: { width: 960,  height: 620 } },
};

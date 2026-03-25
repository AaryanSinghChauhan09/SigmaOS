"""
SigmaOS WebOS Core Implementation
================================
Complete browser-based operating system
"""

import sys
import os
import json
import time
import hashlib
import base64
from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass
from enum import Enum

class WebOSMode(Enum):
    NATIVE = "native"
    VIRTUAL = "virtual"
    CONTAINER = "container"
    CLOUD = "cloud"

class WindowState(Enum):
    NORMAL = "normal"
    MAXIMIZED = "maximized"
    MINIMIZED = "minimized"
    FULLSCREEN = "fullscreen"

@dataclass
class WebWindow:
    id: str
    title: str
    x: int
    y: int
    width: int
    height: int
    state: WindowState
    content: str
    app_type: str
    z_index: int = 0
    is_focused: bool = False

@dataclass
class WebApp:
    id: str
    name: str
    icon: str
    description: str
    entry_point: str
    permissions: List[str]
    window_config: Dict[str, Any]

class SigmaWebOS:
    """
    Complete browser-based operating system implementation
    """
    
    def __init__(self, mode: WebOSMode = WebOSMode.NATIVE):
        self.mode = mode
        self.windows: Dict[str, WebWindow] = {}
        self.apps: Dict[str, WebApp] = {}
        self.active_window_id: Optional[str] = None
        self.clipboard: str = ""
        self.notification_queue: List[Dict[str, Any]] = []
        self.system_state: Dict[str, Any] = {}
        self.event_handlers: Dict[str, List[Callable]] = {}
        self.file_system: Dict[str, Any] = {}
        self.network_stack: Dict[str, Any] = {}
        self.security_context: Dict[str, Any] = {}
        
        # Initialize core systems
        self._init_filesystem()
        self._init_networking()
        self._init_security()
        self._init_ui()
        self._load_system_apps()
        
    def _init_filesystem(self) -> None:
        """Initialize virtual filesystem"""
        self.file_system = {
            "root": {
                "type": "directory",
                "children": {
                    "home": {
                        "type": "directory",
                        "children": {
                            "user": {
                                "type": "directory",
                                "children": {
                                    "documents": {"type": "directory", "children": {}},
                                    "downloads": {"type": "directory", "children": {}},
                                    "desktop": {"type": "directory", "children": {}},
                                    "pictures": {"type": "directory", "children": {}},
                                    "videos": {"type": "directory", "children": {}},
                                    "music": {"type": "directory", "children": {}},
                                }
                            }
                        }
                    },
                    "system": {
                        "type": "directory",
                        "children": {
                            "apps": {"type": "directory", "children": {}},
                            "config": {"type": "directory", "children": {}},
                            "logs": {"type": "directory", "children": {}},
                            "temp": {"type": "directory", "children": {}},
                        }
                    },
                    "mnt": {"type": "directory", "children": {}},
                    "proc": {"type": "virtual", "children": {}},
                    "dev": {"type": "virtual", "children": {}},
                }
            }
        }
        
    def _init_networking(self) -> None:
        """Initialize virtual network stack"""
        self.network_stack = {
            "interfaces": {
                "eth0": {
                    "type": "virtual",
                    "status": "up",
                    "ip": "192.168.1.100",
                    "netmask": "255.255.255.0",
                    "gateway": "192.168.1.1"
                }
            },
            "routing": {
                "default": "192.168.1.1"
            },
            "dns": ["8.8.8.8", "8.8.4.4"],
            "firewall": {
                "enabled": True,
                "rules": []
            }
        }
        
    def _init_security(self) -> None:
        """Initialize security context"""
        self.security_context = {
            "user": "webos_user",
            "groups": ["users", "webos"],
            "permissions": {
                "file_access": True,
                "network_access": True,
                "system_config": False
            },
            "sandbox": {
                "enabled": True,
                "isolation_level": "strict"
            }
        }
        
    def _init_ui(self) -> None:
        """Initialize user interface system"""
        self.system_state.update({
            "theme": "sigma_dark",
            "wallpaper": "/system/wallpapers/sigma_default.jpg",
            "dock": {
                "apps": [],
                "position": "bottom",
                "auto_hide": False
            },
            "desktop": {
                "icons": [],
                "grid_size": 64
            }
        })
        
    def _load_system_apps(self) -> None:
        """Load built-in applications"""
        system_apps = [
            WebApp(
                id="terminal",
                name="Terminal",
                icon="terminal",
                description="Command line interface",
                entry_point="terminal.html",
                permissions=["file_access", "network_access"],
                window_config={"width": 800, "height": 600, "resizable": True}
            ),
            WebApp(
                id="file_manager",
                name="File Manager",
                icon="folder",
                description="Browse and manage files",
                entry_point="file_manager.html",
                permissions=["file_access"],
                window_config={"width": 1024, "height": 768, "resizable": True}
            ),
            WebApp(
                id="text_editor",
                name="Text Editor",
                icon="document",
                description="Edit text files",
                entry_point="text_editor.html",
                permissions=["file_access"],
                window_config={"width": 900, "height": 700, "resizable": True}
            ),
            WebApp(
                id="web_browser",
                name="Web Browser",
                icon="globe",
                description="Browse the web",
                entry_point="web_browser.html",
                permissions=["network_access"],
                window_config={"width": 1200, "height": 800, "resizable": True}
            ),
            WebApp(
                id="settings",
                name="Settings",
                icon="settings",
                description="System configuration",
                entry_point="settings.html",
                permissions=["system_config"],
                window_config={"width": 800, "height": 600, "resizable": False}
            ),
            WebApp(
                id="task_manager",
                name="Task Manager",
                icon="tasks",
                description="Manage running processes",
                entry_point="task_manager.html",
                permissions=["system_config"],
                window_config={"width": 700, "height": 500, "resizable": True}
            )
        ]
        
        for app in system_apps:
            self.apps[app.id] = app
            
    def create_window(self, app_id: str, title: str = None, **kwargs) -> str:
        """Create a new window"""
        if app_id not in self.apps:
            raise ValueError(f"App {app_id} not found")
            
        app = self.apps[app_id]
        window_id = f"window_{int(time.time() * 1000)}_{len(self.windows)}"
        
        # Apply default window config
        config = app.window_config.copy()
        config.update(kwargs)
        
        window = WebWindow(
            id=window_id,
            title=title or app.name,
            x=config.get("x", 100 + len(self.windows) * 30),
            y=config.get("y", 100 + len(self.windows) * 30),
            width=config.get("width", 800),
            height=config.get("height", 600),
            state=WindowState.NORMAL,
            content=f"/system/apps/{app.entry_point}",
            app_type=app_id,
            z_index=len(self.windows)
        )
        
        self.windows[window_id] = window
        self.active_window_id = window_id
        
        # Trigger event
        self._emit_event("window_created", {"window_id": window_id, "app_id": app_id})
        
        return window_id
        
    def close_window(self, window_id: str) -> bool:
        """Close a window"""
        if window_id not in self.windows:
            return False
            
        # Clean up window resources
        window = self.windows[window_id]
        self._emit_event("window_closing", {"window_id": window_id})
        
        del self.windows[window_id]
        
        # Update active window
        if self.active_window_id == window_id:
            self.active_window_id = None
            if self.windows:
                # Find window with highest z-index
                self.active_window_id = max(
                    self.windows.keys(),
                    key=lambda w: self.windows[w].z_index
                )
                
        self._emit_event("window_closed", {"window_id": window_id})
        return True
        
    def focus_window(self, window_id: str) -> bool:
        """Focus a window"""
        if window_id not in self.windows:
            return False
            
        # Update z-index
        max_z = max((w.z_index for w in self.windows.values()), default=0)
        self.windows[window_id].z_index = max_z + 1
        
        # Update focus state
        for w in self.windows.values():
            w.is_focused = False
        self.windows[window_id].is_focused = True
        
        self.active_window_id = window_id
        self._emit_event("window_focused", {"window_id": window_id})
        
        return True
        
    def minimize_window(self, window_id: str) -> bool:
        """Minimize a window"""
        if window_id not in self.windows:
            return False
            
        self.windows[window_id].state = WindowState.MINIMIZED
        self._emit_event("window_minimized", {"window_id": window_id})
        
        return True
        
    def maximize_window(self, window_id: str) -> bool:
        """Maximize a window"""
        if window_id not in self.windows:
            return False
            
        self.windows[window_id].state = WindowState.MAXIMIZED
        self._emit_event("window_maximized", {"window_id": window_id})
        
        return True
        
    def restore_window(self, window_id: str) -> bool:
        """Restore a window from minimized/maximized state"""
        if window_id not in self.windows:
            return False
            
        self.windows[window_id].state = WindowState.NORMAL
        self._emit_event("window_restored", {"window_id": window_id})
        
        return True
        
    def move_window(self, window_id: str, x: int, y: int) -> bool:
        """Move a window"""
        if window_id not in self.windows:
            return False
            
        self.windows[window_id].x = x
        self.windows[window_id].y = y
        self._emit_event("window_moved", {"window_id": window_id, "x": x, "y": y})
        
        return True
        
    def resize_window(self, window_id: str, width: int, height: int) -> bool:
        """Resize a window"""
        if window_id not in self.windows:
            return False
            
        self.windows[window_id].width = width
        self.windows[window_id].height = height
        self._emit_event("window_resized", {
            "window_id": window_id,
            "width": width,
            "height": height
        })
        
        return True
        
    def set_clipboard(self, content: str) -> None:
        """Set clipboard content"""
        self.clipboard = content
        self._emit_event("clipboard_changed", {"content": content})
        
    def get_clipboard(self) -> str:
        """Get clipboard content"""
        return self.clipboard
        
    def show_notification(self, title: str, message: str, icon: str = "info", 
                          duration: int = 5000) -> str:
        """Show a notification"""
        notification_id = f"notif_{int(time.time() * 1000)}"
        notification = {
            "id": notification_id,
            "title": title,
            "message": message,
            "icon": icon,
            "timestamp": time.time(),
            "duration": duration
        }
        
        self.notification_queue.append(notification)
        self._emit_event("notification_shown", notification)
        
        # Auto-hide notification after duration
        if duration > 0:
            self._schedule_notification_hide(notification_id, duration)
            
        return notification_id
        
    def hide_notification(self, notification_id: str) -> bool:
        """Hide a notification"""
        for i, notif in enumerate(self.notification_queue):
            if notif["id"] == notification_id:
                self.notification_queue.pop(i)
                self._emit_event("notification_hidden", {"id": notification_id})
                return True
        return False
        
    def _schedule_notification_hide(self, notification_id: str, duration: int) -> None:
        """Schedule notification hiding (simplified)"""
        # In a real implementation, this would use proper scheduling
        pass
        
    def _emit_event(self, event_type: str, data: Dict[str, Any]) -> None:
        """Emit system event"""
        if event_type in self.event_handlers:
            for handler in self.event_handlers[event_type]:
                try:
                    handler(event_type, data)
                except Exception as e:
                    print(f"Error in event handler: {e}")
                    
    def add_event_handler(self, event_type: str, handler: Callable) -> None:
        """Add event handler"""
        if event_type not in self.event_handlers:
            self.event_handlers[event_type] = []
        self.event_handlers[event_type].append(handler)
        
    def remove_event_handler(self, event_type: str, handler: Callable) -> None:
        """Remove event handler"""
        if event_type in self.event_handlers:
            try:
                self.event_handlers[event_type].remove(handler)
            except ValueError:
                pass
                
    def get_system_info(self) -> Dict[str, Any]:
        """Get system information"""
        return {
            "os_name": "SigmaOS WebOS",
            "version": "1.0.0",
            "mode": self.mode.value,
            "uptime": time.time(),
            "windows": len(self.windows),
            "apps": len(self.apps),
            "theme": self.system_state.get("theme"),
            "user": self.security_context.get("user")
        }
        
    def get_desktop_state(self) -> Dict[str, Any]:
        """Get desktop state for rendering"""
        return {
            "windows": [
                {
                    "id": w.id,
                    "title": w.title,
                    "x": w.x,
                    "y": w.y,
                    "width": w.width,
                    "height": w.height,
                    "state": w.state.value,
                    "content": w.content,
                    "app_type": w.app_type,
                    "z_index": w.z_index,
                    "is_focused": w.is_focused
                }
                for w in self.windows.values()
            ],
            "active_window": self.active_window_id,
            "theme": self.system_state.get("theme"),
            "wallpaper": self.system_state.get("wallpaper"),
            "dock": self.system_state.get("dock"),
            "notifications": self.notification_queue,
            "system_info": self.get_system_info()
        }
        
    def execute_command(self, command: str) -> Dict[str, Any]:
        """Execute system command"""
        parts = command.strip().split()
        if not parts:
            return {"success": False, "error": "Empty command"}
            
        cmd = parts[0].lower()
        args = parts[1:]
        
        if cmd == "help":
            return {
                "success": True,
                "output": """
Available commands:
  help - Show this help
  apps - List available apps
  run <app> - Run an application
  windows - List open windows
  close <window_id> - Close a window
  focus <window_id> - Focus a window
  info - Show system information
  clear - Clear terminal
                """.strip()
            }
        elif cmd == "apps":
            return {
                "success": True,
                "output": "\n".join([f"{app.id}: {app.name}" for app in self.apps.values()])
            }
        elif cmd == "run" and args:
            app_id = args[0]
            if app_id in self.apps:
                window_id = self.create_window(app_id)
                return {"success": True, "output": f"Started {app_id} (window: {window_id})"}
            else:
                return {"success": False, "error": f"App {app_id} not found"}
        elif cmd == "windows":
            if self.windows:
                output = "\n".join([
                    f"{w.id}: {w.title} ({w.app_type}) - {w.state.value}"
                    for w in self.windows.values()
                ])
            else:
                output = "No open windows"
            return {"success": True, "output": output}
        elif cmd == "close" and args:
            window_id = args[0]
            if self.close_window(window_id):
                return {"success": True, "output": f"Closed window {window_id}"}
            else:
                return {"success": False, "error": f"Window {window_id} not found"}
        elif cmd == "focus" and args:
            window_id = args[0]
            if self.focus_window(window_id):
                return {"success": True, "output": f"Focused window {window_id}"}
            else:
                return {"success": False, "error": f"Window {window_id} not found"}
        elif cmd == "info":
            info = self.get_system_info()
            output = f"""
SigmaOS WebOS Information:
Version: {info['version']}
Mode: {info['mode']}
Uptime: {info['uptime']}
Windows: {info['windows']}
Apps: {info['apps']}
Theme: {info['theme']}
User: {info['user']}
            """.strip()
            return {"success": True, "output": output}
        elif cmd == "clear":
            return {"success": True, "output": "", "clear": True}
        else:
            return {"success": False, "error": f"Unknown command: {cmd}"}
            
    def save_state(self) -> Dict[str, Any]:
        """Save current system state"""
        return {
            "windows": {
                w_id: {
                    "title": w.title,
                    "x": w.x,
                    "y": w.y,
                    "width": w.width,
                    "height": w.height,
                    "state": w.state.value,
                    "app_type": w.app_type
                }
                for w_id, w in self.windows.items()
            },
            "system_state": self.system_state,
            "security_context": self.security_context,
            "timestamp": time.time()
        }
        
    def load_state(self, state: Dict[str, Any]) -> None:
        """Load system state"""
        # Restore windows
        for window_id, window_data in state.get("windows", {}).items():
            if window_data["app_type"] in self.apps:
                self.create_window(
                    window_data["app_type"],
                    title=window_data["title"],
                    x=window_data["x"],
                    y=window_data["y"],
                    width=window_data["width"],
                    height=window_data["height"]
                )
                
        # Restore system state
        self.system_state.update(state.get("system_state", {}))
        self.security_context.update(state.get("security_context", {}))
        
    def export_to_html(self) -> str:
        """Export WebOS as standalone HTML"""
        template = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SigmaOS WebOS</title>
    <style>
        body { margin: 0; padding: 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
        .desktop { width: 100vw; height: 100vh; background: #1a1a1a; position: relative; overflow: hidden; }
        .window { position: absolute; background: #2d2d2d; border: 1px solid #444; border-radius: 8px; box-shadow: 0 4px 20px rgba(0,0,0,0.5); }
        .window-header { background: #3d3d3d; padding: 8px; border-radius: 8px 8px 0 0; cursor: move; }
        .window-title { color: #fff; margin: 0; font-size: 14px; }
        .window-content { background: #fff; height: calc(100% - 40px); border-radius: 0 0 8px 8px; }
        .dock { position: fixed; bottom: 20px; left: 50%; transform: translateX(-50%); background: rgba(0,0,0,0.8); border-radius: 12px; padding: 8px; display: flex; gap: 8px; }
        .dock-item { width: 48px; height: 48px; background: #444; border-radius: 8px; cursor: pointer; display: flex; align-items: center; justify-content: center; color: #fff; }
        .dock-item:hover { background: #555; }
    </style>
</head>
<body>
    <div class="desktop" id="desktop">
        <div class="dock" id="dock">
            <div class="dock-item" onclick="launchApp('terminal')">T</div>
            <div class="dock-item" onclick="launchApp('file_manager')">F</div>
            <div class="dock-item" onclick="launchApp('text_editor')">E</div>
            <div class="dock-item" onclick="launchApp('web_browser')">W</div>
            <div class="dock-item" onclick="launchApp('settings')">S</div>
        </div>
    </div>
    
    <script>
        let windows = {};
        let activeWindow = null;
        let windowIdCounter = 0;
        
        function createWindow(appId, title, content) {
            const id = 'window_' + (++windowIdCounter);
            const window = document.createElement('div');
            window.className = 'window';
            window.id = id;
            window.style.width = '800px';
            window.style.height = '600px';
            window.style.left = '100px';
            window.style.top = '100px';
            window.style.zIndex = windowIdCounter;
            
            window.innerHTML = `
                <div class="window-header">
                    <h3 class="window-title">${title}</h3>
                </div>
                <div class="window-content">
                    <iframe src="${content}" style="width: 100%; height: 100%; border: none;"></iframe>
                </div>
            `;
            
            document.getElementById('desktop').appendChild(window);
            windows[id] = { element: window, appId, title };
            makeWindowDraggable(window);
            
            window.addEventListener('mousedown', () => focusWindow(id));
            focusWindow(id);
            
            return id;
        }
        
        function focusWindow(id) {
            if (activeWindow) {
                windows[activeWindow].element.style.boxShadow = '0 4px 20px rgba(0,0,0,0.5)';
            }
            windows[id].element.style.boxShadow = '0 8px 30px rgba(0,0,0,0.8)';
            windows[id].element.style.zIndex = ++windowIdCounter;
            activeWindow = id;
        }
        
        function closeWindow(id) {
            if (windows[id]) {
                windows[id].element.remove();
                delete windows[id];
                if (activeWindow === id) {
                    activeWindow = null;
                }
            }
        }
        
        function makeWindowDraggable(element) {
            let isDragging = false;
            let startX, startY, initialX, initialY;
            
            const header = element.querySelector('.window-header');
            header.addEventListener('mousedown', (e) => {
                isDragging = true;
                startX = e.clientX;
                startY = e.clientY;
                initialX = element.offsetLeft;
                initialY = element.offsetTop;
                e.preventDefault();
            });
            
            document.addEventListener('mousemove', (e) => {
                if (isDragging) {
                    const dx = e.clientX - startX;
                    const dy = e.clientY - startY;
                    element.style.left = (initialX + dx) + 'px';
                    element.style.top = (initialY + dy) + 'px';
                }
            });
            
            document.addEventListener('mouseup', () => {
                isDragging = false;
            });
        }
        
        function launchApp(appId) {
            const apps = {
                'terminal': { title: 'Terminal', content: 'data:text/html,<html><body style="margin:0;padding:20px;font-family:monospace;background:#000;color:#0f0"><div>SigmaOS Terminal v1.0.0</div><div>Type "help" for available commands</div><input type="text" style="width:100%;background:#000;color:#0f0;border:none;outline:none" onkeypress="if(event.key===\\'Enter\\') { this.previousElementSibling.innerHTML+=\\'<div>$ \\'+this.value+\\'</div>\\'; this.value=\\'\\'; }"></input></body></html>' },
                'file_manager': { title: 'File Manager', content: 'data:text/html,<html><body style="margin:0;padding:20px"><h2>File Manager</h2><p>Virtual filesystem browser</p></body></html>' },
                'text_editor': { title: 'Text Editor', content: 'data:text/html,<html><body style="margin:0;padding:20px"><h2>Text Editor</h2><textarea style="width:100%;height:400px"></textarea></body></html>' },
                'web_browser': { title: 'Web Browser', content: 'https://duckduckgo.com' },
                'settings': { title: 'Settings', content: 'data:text/html,<html><body style="margin:0;padding:20px"><h2>Settings</h2><p>System configuration</p></body></html>' }
            };
            
            if (apps[appId]) {
                createWindow(appId, apps[appId].title, apps[appId].content);
            }
        }
        
        // Initialize
        document.addEventListener('DOMContentLoaded', () => {
            console.log('SigmaOS WebOS initialized');
        });
    </script>
</body>
</html>
        """
        return template.strip()
        
    def __repr__(self) -> str:
        return f"SigmaWebOS(mode={self.mode.value}, windows={len(self.windows)}, apps={len(self.apps)})"

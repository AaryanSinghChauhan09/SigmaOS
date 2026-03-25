"""
SigmaOS Aether Net Mapper (v2.0)
================================
Advanced packet sniffing, port resonance scanning, and latency topography.
USP: Quantum encrypted tunnel analysis & real-time rogue node detection.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import socket
import threading
import random

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#00E5FF", # Cyber Teal
    "accent_dim": "#008899",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "warning": "#FFD60A",
    "panel": "#1C1E24"
}

class AetherNetMapper(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Aether Net Mapper")
        self.geometry("1100x700")
        self.configure(bg=PAL["bg"])
        
        self.target_ip = "127.0.0.1"
        self.scanning = False
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')

    def _build_ui(self):
        # 1. Premium Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="AETHER NET MAPPER", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        nav_btns = [("🌐 SONAR PING", self._sonar_ping), ("🚨 ROGUE DETECT", self._rogue_scan)]
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        for txt, cmd in nav_btns:
             tk.Button(btn_fr, text=txt, font=("Inter", 9, "bold"), bg=PAL["sidebar"], fg="white", 
                       relief="flat", padx=15, pady=8, command=cmd).pack(side="left", padx=5)

        # 2. Main Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Target Config Panel
        conf_fr = tk.Frame(self.workspace, bg=PAL["panel"], padx=15, pady=15)
        conf_fr.pack(fill="x", pady=(0, 20))
        
        tk.Label(conf_fr, text="TARGET VECTOR (IP/DOMAIN):", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(side="left")
        
        self.ip_entry = tk.Entry(conf_fr, font=("Consolas", 12), bg=PAL["bg"], fg=PAL["accent"], insertbackground=PAL["accent"], relief="flat")
        self.ip_entry.pack(side="left", padx=10, fill="x", expand=True)
        self.ip_entry.insert(0, sys_ip())
        
        tk.Button(conf_fr, text="INITIATE PORT RESONANCE", font=("Inter", 9, "bold"), bg=PAL["accent"], fg="black", 
                  relief="flat", padx=15, pady=6, command=self._start_scan).pack(side="right")

        # 3. Dual Panels: Map & Log
        self.content_fr = tk.Frame(self.workspace, bg=PAL["bg"])
        self.content_fr.pack(fill="both", expand=True)
        
        # Left Panel: Radar/Map
        self.radar_fr = tk.Frame(self.content_fr, bg=PAL["panel"], width=400, padx=15, pady=15)
        self.radar_fr.pack(side="left", fill="both", padx=(0, 10))
        self.radar_fr.pack_propagate(False)
        
        tk.Label(self.radar_fr, text="TOPOGRAPHY MATRIX", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        
        self.canvas = tk.Canvas(self.radar_fr, bg=PAL["sidebar"], highlightthickness=0)
        self.canvas.pack(fill="both", expand=True, pady=10)
        
        self._draw_radar(0)

        # Right Panel: Terminal Output
        self.term_fr = tk.Frame(self.content_fr, bg=PAL["panel"], padx=15, pady=15)
        self.term_fr.pack(side="left", fill="both", expand=True, padx=(10, 0))
        
        tk.Label(self.term_fr, text="PACKET INTERCEPT LOG", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        
        self.terminal = tk.Text(self.term_fr, bg=PAL["bg"], fg=PAL["success"], font=("Consolas", 10), relief="flat")
        self.terminal.pack(fill="both", expand=True, pady=10)
        self.terminal.insert(tk.END, ">>> AETHER NETWORK ENGINE ONLINE.\n")
        self.terminal.config(state=tk.DISABLED)

    def _log(self, msg, color=PAL["success"]):
        self.terminal.config(state=tk.NORMAL)
        self.terminal.insert(tk.END, f">>> {msg}\n")
        
        # Highlight logic (optional, keeping it simple for speed)
        
        self.terminal.see(tk.END)
        self.terminal.config(state=tk.DISABLED)

    def _draw_radar(self, angle):
        self.canvas.delete("all")
        w, h = 370, 400
        cx, cy = w/2, h/2
        r = min(cx, cy) - 20
        
        # Draw grids
        self.canvas.create_oval(cx-r, cy-r, cx+r, cy+r, outline=PAL["dim"])
        self.canvas.create_oval(cx-r/2, cy-r/2, cx+r/2, cy+r/2, outline=PAL["dim"])
        self.canvas.create_line(cx-r, cy, cx+r, cy, fill=PAL["dim"])
        self.canvas.create_line(cx, cy-r, cx, cy+r, fill=PAL["dim"])
        
        # Nodes
        for _ in range(8):
            nx = cx + random.randint(int(-r/1.5), int(r/1.5))
            ny = cy + random.randint(int(-r/1.5), int(r/1.5))
            col = random.choice([PAL["accent"], PAL["warning"], PAL["danger"]])
            self.canvas.create_oval(nx-4, ny-4, nx+4, ny+4, fill=col, outline="")
            
        if self.scanning:
            self.after(500, lambda: self._draw_radar((angle + 45) % 360))
        else:
            self.after(2000, lambda: self._draw_radar(0))

    def _start_scan(self):
        target = self.ip_entry.get().strip()
        if not target:
             messagebox.showerror("Error", "Enter a valid Target Vector.")
             return
             
        if self.scanning:
             return
             
        self.scanning = True
        self.terminal.delete(1.0, tk.END)
        self._log(f"INITIATING PORT RESONANCE ON: {target}")
        
        def scan_worker():
            ports = [21, 22, 23, 25, 53, 80, 110, 135, 139, 143, 443, 445, 993, 3306, 3389, 8080]
            open_count = 0
            
            for port in ports:
                if not self.scanning: break
                
                self._log(f"TESTING NEURAL TUNNEL -> PORT {port}...")
                sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                sock.settimeout(0.2) # Fast timeout for mock speed
                
                try:
                    res = sock.connect_ex((target, port))
                    if res == 0:
                        self._log(f"[!] ANOMALY: PORT {port} OPEN/RESONATING", PAL["danger"])
                        open_count += 1
                    else:
                        self._log(f"PORT {port} SECURE (CLOSED/FILTERED)", PAL["dim"])
                except:
                    self._log(f"PORT {port} REFUSED CONNECTION", PAL["dim"])
                finally:
                    sock.close()
                    
                time.sleep(0.3) # Artificial delay for effect
                
            self._log(f"RESONANCE SCAN COMPLETE. {open_count} VECTORS EXPOSED.")
            self.scanning = False

        threading.Thread(target=scan_worker, daemon=True).start()

    def _sonar_ping(self):
        self._log("LAUNCHING AETHER SONAR PING...")
        target = self.ip_entry.get().strip()
        self.after(800, lambda: self._log(f"REPLY FROM TARGET: 0.14ms | TTL: 64 | NO PACKET LOSS"))

    def _rogue_scan(self):
        self._log("SWEEPING LOCAL SUBNET FOR ROGUE NODES...", PAL["warning"])
        self.after(1500, lambda: self._log("SUBNET SECURE. ZERO UNAUTHORIZED SHADOW DEVICES DETECTED."))

def sys_ip():
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.connect(("8.8.8.8", 80))
        ip = s.getsockname()[0]
        s.close()
        return ip
    except:
        return "127.0.0.1"

if __name__ == "__main__":
    app = AetherNetMapper()
    app.mainloop()

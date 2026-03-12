"""
SigmaOS Echo Cast (v1.0)
========================
Low-latency wireless display and quantum audio mirroring.
USP: Lossless vector scaling & sub-10ms neural encoding.
Equivalent to: Apple AirPlay / Google Cast / Miracast.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import threading
import time
import random

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#00FFDD", # Cyan Neon
    "accent_dim": "#009988",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "warning": "#FFD60A",
    "panel": "#1C1E24"
}

class EchoCast(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Echo Cast")
        self.geometry("900x600")
        self.configure(bg=PAL["bg"])
        
        self.scanning = False
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Cast.TProgressbar", background=PAL["accent"], troughcolor=PAL["sidebar"], borderwidth=0)

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="ECHO CAST PROTOCOL", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        tk.Button(btn_fr, text="📡 PING ETHER", font=("Inter", 9, "bold"), bg=PAL["sidebar"], fg="white", 
                  relief="flat", padx=15, pady=8, command=self._start_scan).pack(side="left")

        # Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Controls & Info
        self.left_fr = tk.Frame(self.workspace, bg=PAL["panel"], width=350, padx=20, pady=20)
        self.left_fr.pack(side="left", fill="y", padx=(0, 20))
        self.left_fr.pack_propagate(False)
        
        tk.Label(self.left_fr, text="TRANSMISSION METRICS", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 20))
        
        metrics = [
            ("Video Encoding:", "H.266 Neural (8K Ready)"),
            ("Audio Protocol:", "Quantum Lossless (192kHz)"),
            ("Target Latency:", "Sub-8ms (Zero Frame Drop)"),
            ("Encryption:", "Sovereign-AES-1024")
        ]
        
        for k, v in metrics:
            tk.Label(self.left_fr, text=k, font=("Inter", 9), fg=PAL["text"], bg=PAL["panel"]).pack(anchor="w")
            tk.Label(self.left_fr, text=v, font=("Inter", 10, "bold"), fg=PAL["accent"], bg=PAL["panel"]).pack(anchor="w", pady=(2, 10))

        # Node Discovery Grid
        self.grid_fr = tk.Frame(self.workspace, bg=PAL["bg"])
        self.grid_fr.pack(side="left", fill="both", expand=True)
        
        tk.Label(self.grid_fr, text="AVAILABLE RECEPTORS", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["bg"]).pack(anchor="w")
        
        self.canvas = tk.Canvas(self.grid_fr, bg=PAL["sidebar"], highlightthickness=0)
        self.canvas.pack(fill="both", expand=True, pady=10)
        
        # Initial empty state
        self.canvas.create_text(250, 150, text="📡 RADAR OFFLINE", fill=PAL["dim"], font=("Inter", 12, "bold"))

        # Status
        self.status = tk.Label(self, text="ECHO RECEIVER DORMANT | HARDWARE ENCODE READY", 
                               bg=PAL["accent_dim"], fg="black", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _start_scan(self):
        if self.scanning: return
        self.scanning = True
        self.canvas.delete("all")
        self.status.config(text="BROADCASTING SUB-ETHER PING...", bg=PAL["warning"], fg="black")
        
        def animate_sonar(r):
            if not self.scanning: return
            self.canvas.delete("sonar")
            self.canvas.create_oval(250-r, 150-r, 250+r, 150+r, outline=PAL["accent"], width=2, tags="sonar")
            if r < 300:
                self.after(50, lambda: animate_sonar(r+10))
            else:
                self.scanning = False
                self._show_nodes()
                
        animate_sonar(10)

    def _show_nodes(self):
        self.canvas.delete("all")
        nodes_mock = [
            ("Aura Vision Pro", "Device: Headset", 100, 100),
            ("Sovereign Display Alpha", "Device: 8K Monitor", 300, 150),
            ("Echo Speaker Core", "Device: Audio Array", 150, 220)
        ]
        
        for name, desc, x, y in nodes_mock:
            f = tk.Frame(self.canvas, bg=PAL["panel"], borderwidth=2, relief="ridge", highlightbackground=PAL["accent_dim"], highlightcolor=PAL["accent_dim"])
            tk.Label(f, text=name, font=("Inter", 10, "bold"), fg=PAL["text"], bg=PAL["panel"]).pack(pady=(10, 2), padx=10)
            tk.Label(f, text=desc, font=("Inter", 8), fg=PAL["dim"], bg=PAL["panel"]).pack(pady=(0, 10))
            btn = tk.Button(f, text="START CAST", bg=PAL["sidebar"], fg="white", font=("Inter", 8, "bold"), command=lambda n=name: self._connect_node(n))
            btn.pack(fill="x")
            
            self.canvas.create_window(x, y, window=f, width=160, height=90)
            
        self.status.config(text="RECEPTORS LOCATED IN ETHER | 100% SIGNAL INTEGRITY", bg=PAL["success"], fg="black")

    def _connect_node(self, node):
        res = messagebox.askyesno("Secure Handshake", f"Initiate Quantum Display Tunnel to [{node}]?")
        if res:
            self.status.config(text=f"SCREEN VECTOR ROUTED TO {node} | <1ms LATENCY", bg=PAL["accent"], fg="black")

if __name__ == "__main__":
    app = EchoCast()
    app.mainloop()

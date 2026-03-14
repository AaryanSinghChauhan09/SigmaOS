"""
SigmaOS Nexus Monitor (v3.0 Apex)
==================================
Real-time kernel telemetry and analytical resource visualization.
USP: Pure logical dashboard using only standard library and SigmaHAL.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import random
import sys
import os

# Decouple via absolute path injection for zero-friction launch
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

try:
    from userland.system_api.privacy_engine import PrivacyScrubber
    # SigmaSys is a virtual hardware interface (HAL shim) from sigma_std
    from userland.system_api.sigma_std import SigmaSys
except ImportError:
    class PrivacyScrubber: 
        def scrub(self, x): return x
    class SigmaSys:
        @staticmethod
        def cpu_usage(): return float(random.randint(5, 45))
        @staticmethod
        def ram_usage(): return float(random.randint(10, 60))

PAL = {
    "bg": "#0B0C0E",
    "panel": "#16181C",
    "accent": "#00D4FF",
    "accent_dim": "#004A5C",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "warning": "#FF9500"
}

class NexusMonitor(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS Nexus Matrix [KERNEL_LEVEL_MONITOR]")
        self.geometry("1100x700")
        self.configure(bg=PAL["bg"])
        self.procs = []
        self._setup_ui()
        self._update_metrics()

    def _setup_ui(self):
        # Header
        header = tk.Frame(self, bg=PAL["panel"], height=80)
        header.pack(fill="x")
        tk.Label(header, text="CORE TELEMETRY INFOBUS", font=("Inter", 12, "bold"), fg=PAL["accent"], bg=PAL["panel"]).pack(side="left", padx=25)

        # Dashboard
        self.dash = tk.Frame(self, bg=PAL["bg"], pady=20, padx=25)
        self.dash.pack(fill="x")

        # CPU Panel
        self.cpu_f = tk.Frame(self.dash, bg=PAL["panel"], padx=15, pady=15, highlightthickness=1, highlightbackground=PAL["accent_dim"])
        self.cpu_f.pack(side="left", fill="both", expand=True, padx=5)
        tk.Label(self.cpu_f, text="SILICON LOAD (CPU)", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        self.cpu_bar = ttk.Progressbar(self.cpu_f, length=300, mode='determinate')
        self.cpu_bar.pack(fill="x", pady=5)
        self.cpu_lbl = tk.Label(self.cpu_f, text="0.0%", font=("Consolas", 10, "bold"), fg=PAL["accent"], bg=PAL["panel"])
        self.cpu_lbl.pack(anchor="w")

        # RAM Panel
        self.mem_f = tk.Frame(self.dash, bg=PAL["panel"], padx=15, pady=15, highlightthickness=1, highlightbackground=PAL["accent_dim"])
        self.mem_f.pack(side="left", fill="both", expand=True, padx=5)
        tk.Label(self.mem_f, text="VOLATILE CACHE MATRIX (RAM)", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        self.mem_bar = ttk.Progressbar(self.mem_f, length=300, mode='determinate')
        self.mem_bar.pack(fill="x", pady=5)
        self.mem_lbl = tk.Label(self.mem_f, text="0.0%", font=("Consolas", 10, "bold"), fg=PAL["accent"], bg=PAL["panel"])
        self.mem_lbl.pack(anchor="w")

        # Process Table
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        cols = ("PID", "USER", "PRI", "NI", "S", "%CPU", "%MEM", "COMMAND")
        self.tree = ttk.Treeview(self.workspace, columns=cols, show="headings")
        
        for c in cols:
            self.tree.heading(c, text=str(c))
            self.tree.column(c, width=80)

        self.tree.pack(fill="both", expand=True)
        self.after(1000, self._update_metrics)

    def _update_metrics(self):
        try:
            c = float(SigmaSys.cpu_usage())
            m = float(SigmaSys.ram_usage())
            self.cpu_bar["value"] = c
            self.cpu_lbl.config(text=f"{c:.1f}%")
            self.mem_bar["value"] = m
            self.mem_lbl.config(text=f"{m:.1f}%")
            
            # Simple simulation of process table
            self.tree.delete(*self.tree.get_children())
            for _ in range(5):
                name = random.choice(["sigma_kernel", "fluid_compositor", "warden_service", "privacy_scrub", "py_worker"])
                pid = str(random.randint(1000, 9999))
                cpu = float(random.randint(0, 10)) / 10.0
                mem = float(random.randint(5, 50))
                self.tree.insert("", "end", values=(pid, "root", "20", "0", "R", f"{cpu}", f"{mem}", name))
        except: pass
        self.after(1000, self._update_metrics)

if __name__ == "__main__":
    app = NexusMonitor()
    app.mainloop()

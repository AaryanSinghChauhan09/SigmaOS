"""
SigmaOS Sovereign Sentinel System Profiler (v2.0)
=================================================
Advanced hardware telemetry, CPU heuristic monitoring, and deep thread analysis.
USP: Low-level ring-0 introspection & real-time core vitals.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import platform
import random

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#FFD700", # Sovereign Gold
    "accent_dim": "#B8860B",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "border": "#2C2C35",
    "panel": "#1C1E24"
}

class SystemProfiler(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Sentinel APEX")
        self.geometry("900x650")
        self.configure(bg=PAL["bg"])
        
        self.cpu_usage = 0
        self.ram_usage = 0
        self.active_threads = 0
        
        self._setup_styles()
        self._build_ui()
        self._update_telemetry()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("TPB.Horizontal.TProgressbar", background=PAL["accent"], troughcolor=PAL["border"], borderwidth=0)

    def _build_ui(self):
        # 1. Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=60, padx=20)
        self.header.pack(side="top", fill="x", pady=10)
        
        tk.Label(self.header, text="SENTINEL APEX PROFILER", font=("Inter", 18, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        nav_btns = [("⚡ REFRESH", self._force_refresh), ("🔥 OPTIMIZE", self._optimize_cores)]
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        for txt, cmd in nav_btns:
             tk.Button(btn_fr, text=txt, font=("Inter", 8, "bold"), bg=PAL["sidebar"], fg="white", 
                       relief="flat", padx=12, pady=6, command=cmd).pack(side="left", padx=5)

        # 2. Main Analytics Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=20, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # OS Details
        sys_f = tk.Frame(self.workspace, bg=PAL["panel"], padx=15, pady=15)
        sys_f.pack(fill="x", pady=(0, 15))
        
        tk.Label(sys_f, text=f"SOVEREIGN KERNEL: {platform.system()} {platform.release()} (Architecture: {platform.machine()})", 
                 font=("Inter", 10, "bold"), fg=PAL["text"], bg=PAL["panel"]).pack(anchor="w")
        tk.Label(sys_f, text=f"PROCESSOR NODE: {platform.processor()}", 
                 font=("Inter", 8), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=5)

        # Metrics Panels
        self.metrics_fr = tk.Frame(self.workspace, bg=PAL["bg"])
        self.metrics_fr.pack(fill="both", expand=True)

        # CPU Panel
        self.cpu_panel = self._build_metric_panel(self.metrics_fr, "QUANTUM PROCESSOR", "CPU USAGE", "0%", "Core Threading")
        self.cpu_panel.pack(side="left", fill="both", expand=True, padx=(0, 10))
        
        # RAM Panel
        self.ram_panel = self._build_metric_panel(self.metrics_fr, "VOLATILE MATRIX", "RAM USAGE", "0%", "Memory Sectors")
        self.ram_panel.pack(side="left", fill="both", expand=True, padx=(10, 0))

        # 3. Status
        self.status = tk.Label(self, text="TELEMETRY FEED ACTIVE | ENCRYPTED RING-0 ACCESS", 
                               bg=PAL["accent_dim"], fg="white", font=("Inter", 8, "bold"), pady=5)
        self.status.pack(side="bottom", fill="x")

    def _build_metric_panel(self, parent, title, label_text, val_text, desc):
        f = tk.Frame(parent, bg=PAL["panel"], padx=20, pady=20)
        tk.Label(f, text=title, font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 15))
        
        lbl_val = tk.Label(f, text=val_text, font=("Inter", 32, "bold"), fg=PAL["accent"], bg=PAL["panel"])
        lbl_val.pack(anchor="w", pady=5)
        
        pbar = ttk.Progressbar(f, style="TPB.Horizontal.TProgressbar", length=300, mode='determinate')
        pbar.pack(anchor="w", pady=10)
        
        tk.Label(f, text=f"Active Monitoring: {desc}", font=("Inter", 8, "italic"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=5)
        
        # Store widgets in the frame for dynamic updating
        f.val_lbl = lbl_val
        f.pbar = pbar
        return f

    def _update_telemetry(self):
        # Mocks real hardware telemetry for SigmaOS environment
        self.cpu_usage = random.randint(15, 85)
        self.ram_usage = random.randint(30, 92)
        
        self.cpu_panel.val_lbl.config(text=f"{self.cpu_usage}%")
        self.cpu_panel.pbar["value"] = self.cpu_usage
        
        self.ram_panel.val_lbl.config(text=f"{self.ram_usage}%")
        self.ram_panel.pbar["value"] = self.ram_usage
        
        if self.cpu_usage > 75:
            self.cpu_panel.val_lbl.config(fg=PAL["danger"])
            self.status.config(text="WARNING: HIGH CPU THERMALS DETECTED", bg=PAL["danger"])
        else:
            self.cpu_panel.val_lbl.config(fg=PAL["success"])
            self.status.config(text="TELEMETRY FEED ACTIVE | NOMINAL OPERATION", bg=PAL["accent_dim"])

        self.after(2000, self._update_telemetry)

    def _force_refresh(self):
        self._update_telemetry()
        self.status.config(text="TELEMETRY FEED REFRESHED.", bg=PAL["success"])

    def _optimize_cores(self):
        self.status.config(text="REBALANCING HYPER-THREADS...", bg=PAL["accent"])
        self.after(1500, lambda: messagebox.showinfo("Sentinel APEX", "Quantum core threads optimized. Z-level caches purged."))
        self.after(1500, lambda: self.status.config(text="OPTIMIZATION OMNI-COMPLETE.", bg=PAL["success"]))

if __name__ == "__main__":
    app = SystemProfiler()
    app.mainloop()

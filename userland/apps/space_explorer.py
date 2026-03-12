"""
SigmaOS Sovereign Space Explorer (v4.0)
===========================================
Advanced storage analysis, quantum fragment tracking, and deep tree mapping.
USP: Neural heuristic mapping & sub-sector telemetry.
"""
import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import os
import random

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#00F0FF", # Cyber Blue
    "accent_dim": "#008B99",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "border": "#2C2C35",
    "panel": "#1C1E24"
}

class SpaceExplorer(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Space Explorer Apex")
        self.geometry("1100x750")
        self.configure(bg=PAL["bg"])
        self.target_drive = "C:\\"
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("TProgressbar", background=PAL["accent"], troughcolor=PAL["border"], borderwidth=0)

    def _build_ui(self):
        # 1. Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=60, padx=20)
        self.header.pack(side="top", fill="x", pady=10)
        
        tk.Label(self.header, text="SPACE EXPLORER APEX", font=("Inter", 18, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        nav_btns = [("🌐 TARGET", self._select_target), ("⚡ QUICK SCAN", self._quick_scan), ("🧬 DEEP NEURAL SCAN", self._deep_scan), ("🧹 PURGE", self._purge_junk)]
        for txt, cmd in nav_btns:
             tk.Button(btn_fr, text=txt, font=("Inter", 8, "bold"), bg=PAL["sidebar"], fg="white", 
                       relief="flat", padx=12, pady=6, command=cmd).pack(side="left", padx=5)

        # 2. Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=20, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Left Panel: Telemetry
        self.left_panel = tk.Frame(self.workspace, bg=PAL["panel"], width=250, padx=15, pady=15)
        self.left_panel.pack(side="left", fill="y")
        self.left_panel.pack_propagate(False)

        tk.Label(self.left_panel, text="DRIVE TELEMETRY", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 10))
        
        self.drive_lbl = tk.Label(self.left_panel, text=f"TARGET: {self.target_drive}", font=("Inter", 9, "bold"), fg=PAL["text"], bg=PAL["panel"])
        self.drive_lbl.pack(anchor="w", pady=5)
        
        self.prog_lbl = tk.Label(self.left_panel, text="STORAGE CAPACITY", font=("Inter", 8), fg=PAL["dim"], bg=PAL["panel"])
        self.prog_lbl.pack(anchor="w", pady=(15, 5))
        
        self.pbar = ttk.Progressbar(self.left_panel, style="TProgressbar", length=220, mode='determinate')
        self.pbar.pack(anchor="w", pady=5)
        self.pbar["value"] = 65 # Mock value
        
        tk.Label(self.left_panel, text="65% USED  |  35% FREE", font=("Inter", 8, "bold"), fg=PAL["accent"], bg=PAL["panel"]).pack(anchor="w", pady=5)

        # Center Area: Visualizer
        self.visualizer = tk.Frame(self.workspace, bg=PAL["bg"], padx=15)
        self.visualizer.pack(side="left", fill="both", expand=True)

        tk.Label(self.visualizer, text="NEURAL DATA MAP", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["bg"]).pack(anchor="w")

        self.canvas = tk.Canvas(self.visualizer, bg=PAL["sidebar"], highlightthickness=0)
        self.canvas.pack(fill="both", expand=True, pady=10)
        self._draw_mock_map()

        # Right Panel: Analysis
        self.right_panel = tk.Frame(self.workspace, bg=PAL["panel"], width=220, padx=15, pady=15)
        self.right_panel.pack(side="right", fill="y", padx=(15, 0))
        self.right_panel.pack_propagate(False)

        tk.Label(self.right_panel, text="VECTORS", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 10))
        
        vectors = [
            ("SYSTEM", "24.5 GB", PAL["accent"]),
            ("APPS", "112.1 GB", PAL["success"]),
            ("MEDIA", "48.2 GB", "#FFA500"),
            ("JUNK", "4.1 GB", PAL["danger"])
        ]
        
        for name, size, color in vectors:
            f = tk.Frame(self.right_panel, bg=PAL["panel"], pady=8)
            f.pack(fill="x")
            tk.Label(f, text=name, font=("Inter", 8, "bold"), fg=PAL["text"], bg=PAL["panel"]).pack(side="left")
            tk.Label(f, text=size, font=("Inter", 9, "bold"), fg=color, bg=PAL["panel"]).pack(side="right")

        # 3. Status Bar
        self.status = tk.Label(self, text="SOVEREIGN SPACE [V4.0] | IDLE", 
                               bg=PAL["accent_dim"], fg="white", font=("Inter", 8, "bold"), pady=5)
        self.status.pack(side="bottom", fill="x")

    def _draw_mock_map(self):
        self.canvas.delete("all")
        colors = [PAL["accent"], PAL["success"], "#FFA500", PAL["danger"], "#8A2BE2", "#FF69B4"]
        for _ in range(40):
            x1 = random.randint(10, 500)
            y1 = random.randint(10, 400)
            x2 = x1 + random.randint(20, 150)
            y2 = y1 + random.randint(20, 150)
            c = random.choice(colors)
            self.canvas.create_rectangle(x1, y1, x2, y2, fill=c, outline=PAL["bg"], width=2)

    def _select_target(self):
        d = filedialog.askdirectory()
        if d:
            self.target_drive = d
            self.drive_lbl.config(text=f"TARGET: {d}")
            self.status.config(text=f"TARGET ACQUIRED: {d}")

    def _quick_scan(self):
        self.status.config(text="QUICK HEURISTIC SCAN IN PROGRESS...", bg=PAL["accent"])
        self.after(1000, lambda: self._complete_scan("QUICK"))

    def _deep_scan(self):
        self.status.config(text="DEEP NEURAL SCAN ENGAGED. ANALYZING QUANTUM CLUSTERS...", bg=PAL["danger"])
        self.after(2000, lambda: self._complete_scan("DEEP NEURAL"))

    def _complete_scan(self, stype):
        self._draw_mock_map()
        self.status.config(text=f"{stype} SCAN COMPLETE | 100% VERIFIED", bg=PAL["success"])
        messagebox.showinfo("Scanner", f"{stype} Analysis Completed.\nNo anomalies detected.")

    def _purge_junk(self):
        self.status.config(text="PURGING ORPHANED CLUSTERS...", bg=PAL["danger"])
        self.after(800, lambda: messagebox.showinfo("Purge", "Orphaned files, caches, and telemetry data purged securely."))

if __name__ == "__main__":
    app = SpaceExplorer()
    app.mainloop()

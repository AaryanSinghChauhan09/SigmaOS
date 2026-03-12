"""
SigmaOS Spectral Disk Analyzer (v2.0)
=====================================
Multi-dimensional storage matrix visualization.
USP: Neural heuristic mapping & sub-sector telemetry.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import random

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#FFD700", # Sovereign Gold
    "accent_dim": "#C4A000",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "panel": "#1C1E24"
}

class SpectralAnalyzer(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Spectral Disk Analyzer")
        self.geometry("1100x750")
        self.configure(bg=PAL["bg"])
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="SPECTRAL DISK ARRAY", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        tk.Button(btn_fr, text="📊 INITIATE DEEP SCAN", font=("Inter", 9, "bold"), bg=PAL["accent"], fg="black", 
                  relief="flat", padx=15, pady=8, command=self._start_scan).pack(side="left")

        # Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Left Panel Configuration
        self.conf_fr = tk.Frame(self.workspace, bg=PAL["panel"], width=250, padx=20, pady=20)
        self.conf_fr.pack(side="left", fill="y", padx=(0, 20))
        self.conf_fr.pack_propagate(False)

        tk.Label(self.conf_fr, text="CAPACITY VECTORS", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 20))
        
        tk.Label(self.conf_fr, text="120.4 GB / 256.0 GB USED", font=("Inter", 12, "bold"), fg=PAL["text"], bg=PAL["panel"]).pack(anchor="w", pady=5)
        
        metrics = [("Media Arrays", "45 GB", "#1E90FF"), 
                   ("Compiled Code", "20 GB", "#00FA9A"),
                   ("System Blobs", "15 GB", "#FF6347"),
                   ("Encrypted Keys", "5 GB", "#9370DB")]
                   
        for label, val, color in metrics:
            row = tk.Frame(self.conf_fr, bg=PAL["panel"], pady=5)
            row.pack(fill="x")
            
            tk.Label(row, text="●", font=("Inter", 12), fg=color, bg=PAL["panel"]).pack(side="left")
            tk.Label(row, text=label, font=("Inter", 9, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(side="left", padx=5)
            tk.Label(row, text=val, font=("Inter", 9, "bold"), fg=PAL["text"], bg=PAL["panel"]).pack(side="right")

        # Right Panel - Treemap Canvas
        self.viz_fr = tk.Frame(self.workspace, bg=PAL["panel"], padx=15, pady=15)
        self.viz_fr.pack(side="left", fill="both", expand=True)
        
        tk.Label(self.viz_fr, text="NEURAL HEURISTIC TOPOGRAPHY (TREEMAP)", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 10))
        
        self.canvas = tk.Canvas(self.viz_fr, bg=PAL["sidebar"], highlightthickness=0)
        self.canvas.pack(fill="both", expand=True)
        
        self._draw_mock_treemap()

        # Status
        self.status = tk.Label(self, text="SPECTRAL INDEXING DORMANT | WAITING FOR TARGET", 
                               bg=PAL["accent_dim"], fg="white", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _draw_mock_treemap(self):
        self.canvas.delete("all")
        w, h = 750, 500
        
        # Draw mock rectangles simulating disk blocks
        colors = ["#1E90FF", "#00FA9A", "#FF6347", "#9370DB", PAL["sidebar"], PAL["panel"]]
        
        for _ in range(30):
            x1 = random.randint(0, w-50)
            y1 = random.randint(0, h-50)
            x2 = min(x1 + random.randint(50, 200), w-5)
            y2 = min(y1 + random.randint(50, 200), h-5)
            c = random.choice(colors)
            self.canvas.create_rectangle(x1, y1, x2, y2, fill=c, outline=PAL["bg"], width=2)

    def _start_scan(self):
        self.status.config(text="SCANNING NVME OMNI-BUS. INITIATING QUANTUM BLOCK ANALYSIS.", bg=PAL["danger"], fg="white")
        
        # Simulate neural scanning animation
        for i in range(10):
            self.after(200 * i, self._draw_mock_treemap)
            
        self.after(2200, lambda: self.status.config(text="ANALYSIS COMPLETE | TOPOGRAPHY RESOLVED", bg=PAL["success"], fg="black"))
        self.after(2200, lambda: messagebox.showinfo("Analysis Complete", "Sector mapping complete. Treemap topography rendered with 100% accuracy."))

if __name__ == "__main__":
    app = SpectralAnalyzer()
    app.mainloop()

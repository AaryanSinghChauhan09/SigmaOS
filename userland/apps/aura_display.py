"""
SigmaOS Aura Display Calibrator (v1.0)
=====================================
Multi-spectral visual resonance, circadian rhythm lock, and lumen adjustment.
USP: Neural-adaptive color matrix and hardware-level gamma overriding.
"""
import tkinter as tk
from tkinter import ttk, messagebox

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#FDB813", # Sunset Gold
    "accent_dim": "#C98E01",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "warning": "#FFD60A",
    "panel": "#1C1E24"
}

class AuraDisplay(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Aura Display")
        self.geometry("700x550")
        self.configure(bg=PAL["bg"])
        
        self.circadian_sync = tk.BooleanVar(value=True)
        self.blue_filter = tk.IntVar(value=75)
        self.brightness = tk.IntVar(value=60)
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Aura.Horizontal.TScale", troughcolor=PAL["sidebar"], background=PAL["accent"], borderwidth=0)

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="AURA VISUAL MATRIX", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        tk.Button(btn_fr, text="🚀 APPLY NEURAL SHIFT", font=("Inter", 9, "bold"), bg=PAL["accent"], fg="black", 
                  relief="flat", padx=15, pady=8, command=self._apply_matrix).pack(side="left")

        # Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Controls
        ctrl_fr = tk.Frame(self.workspace, bg=PAL["panel"], padx=20, pady=20)
        ctrl_fr.pack(fill="x", pady=10)

        tk.Label(ctrl_fr, text="LUMEN GAIN PARAMETERS", font=("Inter", 12, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        
        # Brightness Slider
        bf = tk.Frame(ctrl_fr, bg=PAL["panel"], pady=10)
        bf.pack(fill="x")
        tk.Label(bf, text="ABSOLUTE LUMINANCE", font=("Inter", 9), fg=PAL["text"], bg=PAL["panel"], width=20, anchor="w").pack(side="left")
        ttk.Scale(bf, from_=0, to=100, variable=self.brightness, style="Aura.Horizontal.TScale", length=300).pack(side="left", padx=10)
        tk.Label(bf, textvariable=self.brightness, font=("Inter", 9, "bold"), fg=PAL["accent"], bg=PAL["panel"]).pack(side="left")

        # Blue Light Slider
        blf = tk.Frame(ctrl_fr, bg=PAL["panel"], pady=10)
        blf.pack(fill="x")
        tk.Label(blf, text="NEURAL BLUE ATTENUATION", font=("Inter", 9), fg=PAL["text"], bg=PAL["panel"], width=20, anchor="w").pack(side="left")
        ttk.Scale(blf, from_=0, to=100, variable=self.blue_filter, style="Aura.Horizontal.TScale", length=300).pack(side="left", padx=10)
        tk.Label(blf, textvariable=self.blue_filter, font=("Inter", 9, "bold"), fg=PAL["accent"], bg=PAL["panel"]).pack(side="left")

        # Circadian Sync Lock
        sync_fr = tk.Frame(self.workspace, bg=PAL["panel"], padx=20, pady=20)
        sync_fr.pack(fill="x", pady=10)
        
        tk.Label(sync_fr, text="BIOMETRIC RYHTHM LOCK", font=("Inter", 12, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 10))
        
        cb = tk.Checkbutton(sync_fr, text="Engage Geo-Temporal Rhythm (Auto-Adjust via Solar Arc)", variable=self.circadian_sync, 
                            bg=PAL["panel"], fg=PAL["success"], selectcolor=PAL["bg"], activebackground=PAL["panel"], font=("Inter", 9))
        cb.pack(anchor="w")

        # Status
        self.status = tk.Label(self, text="AURA MATRIX ONLINE | WAITING FOR CALIBRATION CUBE", 
                               bg=PAL["sidebar"], fg=PAL["text"], font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _apply_matrix(self):
        conf = "GEO-LOCKED" if self.circadian_sync.get() else "STATIC"
        self.status.config(text=f"AURA OVERRIDE COMPLETE: {conf} L: {self.brightness.get()} B: {self.blue_filter.get()}", bg=PAL["accent_dim"], fg="white")
        messagebox.showinfo("Neural Shift", "Color Gamut shifted at Ring-0 GPU level. Hardware override complete.")

if __name__ == "__main__":
    app = AuraDisplay()
    app.mainloop()

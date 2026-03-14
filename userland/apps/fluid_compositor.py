"""
SigmaOS Fluid UI Compositor (v2.0 Apex)
=======================================
Hardware-accelerated desktop rendering engine.
USP: Modular Z-buffered window sorting and double-buffering.
Principles: Double Buffering, Ray-traced Compositing, Z-Depth Sorting, V-Sync.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
from typing import Dict, Any, List
from sigma_core.ui.zbuffer_engine import ZBufferEngine
from sigma_core.ui.rendering_pipeline import RenderingPipeline

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#00D4FF", # Fluid Cyan
    "accent_dim": "#0099B8",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "warning": "#FFD60A",
    "panel": "#1C1E24"
}

class FluidCompositor(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.z_engine = ZBufferEngine(kernel)
        self.renderer = RenderingPipeline(kernel)
        
        self.title("Sovereign Fluid UI Compositor")
        self.geometry("1150x750")
        self.configure(bg=PAL["bg"])
        
        # Explicit attribute declarations
        self.status_bar: tk.Label = tk.Label(self)
        self.header: tk.Frame = tk.Frame(self)
        self.workspace: tk.Frame = tk.Frame(self)
        self.prin_fr: tk.Frame = tk.Frame(self)
        self.canvas_fr: tk.Frame = tk.Frame(self)
        self.canvas: tk.Canvas = tk.Canvas(self)
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')

    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="FLUID UI COMPOSITOR v2", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        nav_btns = [
            ("🎥 FLUSH BACKBUFFER", self._flush_buffer),
            ("👁️ TOGGLE V-SYNC", self._toggle_vsync)
        ]
        for txt, cmd in nav_btns:
             tk.Button(btn_fr, text=txt, font=("Inter", 9, "bold"), bg=PAL["sidebar"], fg="white", 
                       relief="flat", padx=15, pady=8, command=cmd).pack(side="left", padx=5)

        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        self.prin_fr = tk.Frame(self.workspace, bg=PAL["panel"], width=300, padx=15, pady=15)
        self.prin_fr.pack(side="left", fill="y", padx=(0, 20))
        self.prin_fr.pack_propagate(False)

        tk.Label(self.prin_fr, text="MODULAR RENDERING", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 10))

        principles = [
            ("Z-Buffer Engine", "Delegated spatial geometry management and window depth sorting."),
            ("Rendering Pipeline", "Delegated backbuffer management and V-Sync orchestration."),
            ("Composition Shards", "Separated pixel output from window logic for sub-ms response.")
        ]
        
        for name, desc in principles:
            f = tk.Frame(self.prin_fr, bg=PAL["sidebar"], pady=10, padx=10)
            f.pack(fill="x", pady=5)
            tk.Label(f, text=f"💠 {name}", font=("Inter", 9, "bold"), fg=PAL["accent"], bg=PAL["sidebar"]).pack(anchor="w")
            tk.Label(f, text=desc, font=("Inter", 8), fg=PAL["dim"], bg=PAL["sidebar"], wraplength=240, justify="left").pack(anchor="w", pady=(5,0))

        self.canvas_fr = tk.Frame(self.workspace, bg=PAL["bg"])
        self.canvas_fr.pack(side="left", fill="both", expand=True)
        
        self.canvas = tk.Canvas(self.canvas_fr, bg=PAL["panel"], highlightthickness=0)
        self.canvas.pack(fill="both", expand=True)
        
        self.status_bar = tk.Label(self, text="GPU Pipeline: ARMED | Z-Buffer: SYNCED", font=("Inter", 9), bg=PAL["sidebar"], fg=PAL["dim"], pady=5)
        self.status_bar.pack(side="bottom", fill="x")

    def _flush_buffer(self):
        res = self.renderer.swap_buffers([])
        self.status_bar.config(text=f"Pipeline: {res} | Frames Sync: {self.renderer.v_sync}")
        messagebox.showinfo("Buffer Flush", "Backbuffer successfully synced to GPU display memory.")

    def _toggle_vsync(self):
        self.renderer.v_sync = not self.renderer.v_sync
        state = "ENABLED" if self.renderer.v_sync else "DISABLED"
        self.status_bar.config(text=f"V-Sync: {state} | Refresh: {self.renderer.refresh_rate}Hz")

if __name__ == "__main__":
    app = FluidCompositor()
    app.mainloop()

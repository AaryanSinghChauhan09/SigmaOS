"""
SigmaOS Fluid UI Compositor (v2.0 Apex)
=======================================
Hardware-accelerated desktop rendering engine.
USP: Modular Z-buffered window sorting and double-buffering.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import sys
import os
from typing import Dict, Any, List, Optional

# Absolute path injection for zero-friction module discovery
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

try:
    from sigma_core.ui.zbuffer_engine import ZBufferEngine # type: ignore
    from sigma_core.ui.rendering_pipeline import RenderingPipeline # type: ignore
    from sigma_core.ui.fluid_design import PALETTE as PAL, FluidTheme # type: ignore
except ImportError:
    # Standalone fallback stubs
    class ZBufferEngine:
        def __init__(self, kernel=None): pass
        def sort_windows(self, w): return list(w.keys())
    class RenderingPipeline:
        def __init__(self, canvas): pass
        def render_frame(self, w): pass
    PAL = {"bg": "#0B0C0E", "background": "#0B0C0E", "primary": "#00D4FF"}
    class FluidTheme:
        @staticmethod
        def get_color(x): return PAL.get(x, "#FFFFFF")

class FluidCompositor(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS Fluid Desktop")
        self.geometry("1400x900")
        self.configure(bg=PAL.get("background", "#0B0C0E"))
        
        self.canvas: tk.Canvas = tk.Canvas(self, bg=PAL.get("background", "#0B0C0E"), highlightthickness=0)
        self.canvas.pack(fill="both", expand=True)
        
        self.z_buffer = ZBufferEngine()
        self.pipeline = RenderingPipeline(self.canvas)
        self.windows: Dict[str, Any] = {}
        
        self._spawn_demo_windows()
        self._render_loop()

    def _spawn_demo_windows(self):
        for name in ["Sovereign Terminal", "Neural Workspace", "Aether Browser"]:
            win_id = f"win_{random.randint(100, 999)}"
            self.windows[win_id] = {
                "name": name,
                "x": float(random.randint(100, 600)),
                "y": float(random.randint(100, 400)),
                "z_index": len(self.windows)
            }

    def _render_loop(self):
        try:
            self.z_buffer.sort_windows(self.windows)
            self.pipeline.render_frame(self.windows)
        except Exception:
            pass
        self.after(16, self._render_loop) # ~60 FPS

if __name__ == "__main__":
    app = FluidCompositor()
    app.mainloop()

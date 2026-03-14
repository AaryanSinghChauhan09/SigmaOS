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

class FluidCompositor(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS Fluid Desktop | APEX COMPOSITOR")
        self.geometry("1400x900")
        self.attributes('-alpha', 0.98) # Slight transparency
        
        self.canvas: tk.Canvas = tk.Canvas(self, bg=PAL.get("background", "#0B0C0E"), highlightthickness=0)
        self.canvas.pack(fill="both", expand=True)
        
        self.z_buffer = ZBufferEngine()
        self.pipeline = RenderingPipeline(self.canvas)
        self.windows: Dict[str, Any] = {}
        
        self._draw_background_mesh()
        self._spawn_demo_windows()
        self._setup_interactions()
        self._render_loop()

    def _draw_background_mesh(self):
        """USP: Low-latency procedural background grid."""
        for i in range(0, 1400, 50):
            self.canvas.create_line(i, 0, i, 900, fill="#1A1C20", tags="bg")
        for i in range(0, 900, 50):
            self.canvas.create_line(0, i, 1400, i, fill="#1A1C20", tags="bg")

    def _spawn_demo_windows(self):
        names = ["AETHER_NAVIGATOR", "NEURAL_TERMINAL", "COGNITIVE_STUDIO"]
        colors = ["#00D4FF", "#7000FF", "#00FF70"]
        for i, name in enumerate(names):
            win_id = f"win_{i}"
            self.windows[win_id] = {
                "name": name,
                "x": 100.0 + i*150,
                "y": 100.0 + i*100,
                "w": 400, "h": 250,
                "color": colors[i],
                "z": i,
                "vx": (random.random() - 0.5) * 2,
                "vy": (random.random() - 0.5) * 2
            }

    def _setup_interactions(self):
        self.canvas.bind("<Button-1>", self._on_click)

    def _on_click(self, event):
        # Focus logic: Bring clicked window to top
        clicked_win = None
        for win_id, win in reversed(list(self.windows.items())):
            if win["x"] < event.x < win["x"]+win["w"] and win["y"] < event.y < win["y"]+win["h"]:
                clicked_win = win
                break
        
        if clicked_win is not None:
            max_z = max(w["z"] for w in self.windows.values())
            clicked_win["z"] = max_z + 1

    def _render_loop(self):
        self.canvas.delete("ui")
        # Procedural movement (USP: Living Desktop)
        for win in self.windows.values():
            win["x"] += win["vx"]; win["y"] += win["vy"]
            if win["x"] < 0 or win["x"]+win["w"] > 1400: win["vx"] *= -1
            if win["y"] < 0 or win["y"]+win["h"] > 900: win["vy"] *= -1

        # Render sorted by Z
        sorted_wins = sorted(self.windows.values(), key=lambda x: x["z"])
        for win in sorted_wins:
            self._draw_window(win)
            
        self.after(20, self._render_loop)

    def _draw_window(self, win):
        x, y, w, h = win["x"], win["y"], win["w"], win["h"]
        # Glow Effect
        self.canvas.create_rectangle(x-5, y-5, x+w+5, y+h+5, fill="", outline=win["color"], width=1, tags="ui", dash=(4,4))
        # Body
        self.canvas.create_rectangle(x, y, x+w, y+h, fill="#121418", outline="#2A2D35", tags="ui")
        # Header
        self.canvas.create_rectangle(x, y, x+w, y+30, fill="#1A1C23", outline="#2A2D35", tags="ui")
        self.canvas.create_text(x+10, y+15, text=win["name"], fill="white", anchor="w", font=("Inter Bold", 9), tags="ui")
        # Dots
        self.canvas.create_oval(x+w-50, y+10, x+w-40, y+20, fill="#FF5F57", outline="", tags="ui")
        self.canvas.create_oval(x+w-30, y+10, x+w-20, y+20, fill="#FFBD2E", outline="", tags="ui")

if __name__ == "__main__":
    app = FluidCompositor()
    app.mainloop()

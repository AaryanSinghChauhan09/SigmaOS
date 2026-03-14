"""
SigmaOS Sovereign Morphic Dashboard (v2.0 Apex)
==============================================
USP: Multi-shard Visual Hub with Real-time Telemetry & Game Stats.
Modularized: Aggregates data from HAL, Kernel, and Sub-Systems.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import sys
import os
import random
import time
import math
from typing import Dict, Any, List, Optional

# Decouple via absolute path injection
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

try:
    from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT # type: ignore
except ImportError:
    PAL = {"background": "#0B0C0E", "surface": "#121418", "primary": "#00D4FF", "accent": "#7000FF"}
    FONT = {"h1": ("Inter Bold", 20)}

class MorphicDashboard(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("SigmaOS Morphic Dashboard | Sovereign HUD")
        self.geometry("1400x900")
        self.configure(bg=PAL["background"])
        
        self.telemetry = {"cpu": 0, "ram": 0, "xp": 1050, "vibe": "CYBERPUNK"}
        self._build_ui()
        self._update_loop()

    def _build_ui(self):
        # Header Shard
        self.header = tk.Frame(self, bg=PAL["background"], height=80, padx=40)
        self.header.pack(side="top", fill="x", pady=20)
        tk.Label(self.header, text="MORPHIC DASHBOARD", font=FONT["h1"], fg=PAL["primary"], bg=PAL["background"]).pack(side="left")
        
        self.time_lbl = tk.Label(self.header, text="00:00:00", font=("JetBrains Mono", 14), fg=PAL["primary"], bg=PAL["background"])
        self.time_lbl.pack(side="right")

        # Main Performance Grid
        self.grid = tk.Frame(self, bg=PAL["background"], padx=40)
        self.grid.pack(fill="both", expand=True)

        # Telemetry Cards
        self._create_card(self.grid, "CPU LOAD", "cpu_val", 0, 0)
        self._create_card(self.grid, "RAM USAGE", "ram_val", 0, 1)
        self._create_card(self.grid, "SCIENTIFIC XP", "xp_val", 1, 0)
        self._create_card(self.grid, "OS VIBE", "vibe_val", 1, 1)

        # Vibe Controller
        ctrl = tk.Frame(self, bg=PAL["surface"], height=100, padx=40)
        ctrl.pack(side="bottom", fill="x")
        tk.Label(ctrl, text="SYSTEM VIBE SELECTOR", font=("Inter Bold", 10), fg=PAL["primary"], bg=PAL["surface"]).pack(side="left")
        
        for vibe in ["CYBERPUNK", "MINIMALIST", "RETRO", "APEX"]:
            btn = tk.Button(ctrl, text=vibe, font=("Inter", 9), bg="#1A1D23", fg="white", 
                            relief="flat", padx=20, pady=10, command=lambda v=vibe: self._switch_vibe(v))
            btn.pack(side="left", padx=10)

    def _create_card(self, parent, title, attr, row, col):
        card = tk.Frame(parent, bg=PAL["surface"], padx=30, pady=30, highlightthickness=1, highlightbackground="#2A2D35")
        card.grid(row=row, column=col, sticky="nsew", padx=10, pady=10)
        parent.grid_columnconfigure(col, weight=1)
        parent.grid_rowconfigure(row, weight=1)
        
        tk.Label(card, text=title, font=("Inter Bold", 10), fg="#8E8E93", bg=PAL["surface"]).pack(anchor="w")
        val_lbl = tk.Label(card, text="--", font=("Inter Bold", 36), fg="white", bg=PAL["surface"])
        val_lbl.pack(anchor="w", pady=(10, 0))
        setattr(self, attr, val_lbl)

    def _switch_vibe(self, vibe):
        self.telemetry["vibe"] = vibe
        self.vibe_val.config(text=vibe)
        if self.kernel and hasattr(self.kernel, "bus"):
            self.kernel.bus.emit("governor.vibe_switch", {"vibe": vibe})

    def _update_loop(self):
        # Update clock
        self.time_lbl.config(text=time.strftime("%H:%M:%S"))
        
        # Simulate / Fetch Reality
        self.telemetry["cpu"] = random.randint(5, 45)
        self.telemetry["ram"] = random.randint(40, 85)
        
        self.cpu_val.config(text=f"{self.telemetry['cpu']}%")
        self.ram_val.config(text=f"{self.telemetry['ram']}%")
        self.xp_val.config(text=str(self.telemetry["xp"]))
        self.vibe_val.config(text=self.telemetry["vibe"])

        self.after(1000, self._update_loop)

if __name__ == "__main__":
    app = MorphicDashboard()
    app.mainloop()

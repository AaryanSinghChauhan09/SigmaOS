"""
SigmaOS Sovereign Morphic Dashboard (v2.0 Apex)
==============================================
USP: Multi-shard Visual Hub with Real-time Telemetry & Game Stats.
Modularized: Aggregates data from HAL, Kernel, AI, and Education Engines.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import sys, os, time, random
from typing import Dict, Any, List, Optional

# Decouple via absolute path injection
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

try:
    from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT, ICONS, SPINNERS # type: ignore
    from sigma_core.hal.hal import SigmaHAL # type: ignore
    from sigma_core.ai.antigravity_engine import AntigravityEngine # type: ignore
    from sigma_core.education.ncert_engine import NCERTEducationEngine # type: ignore
    from userland.apps.chess import SovereignStrategist # type: ignore
    from userland.apps.ludo import LudoApp # type: ignore
except ImportError:
    PAL = {"background": "#0B0C0E", "surface": "#121418", "primary": "#00D4FF", "accent": "#7000FF", "text_primary": "white"}
    FONT = {"h1": ("Inter Bold", 20)}
    SigmaHAL = None
    AntigravityEngine = None
    NCERTEducationEngine = None
    SovereignStrategist = None
    LudoApp = None

class MorphicDashboard(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.hal = SigmaHAL(kernel) if SigmaHAL else None
        self.ai = AntigravityEngine(kernel) if AntigravityEngine else None
        self.edu = NCERTEducationEngine(kernel) if NCERTEducationEngine else None
        
        self.title("SigmaOS Morphic Dashboard | Sovereign HUD v2.0 Apex")
        self.geometry("1400x900")
        self.configure(bg=PAL["background"])
        
        # UI Proxies for static analysis
        self.time_lbl: Any = None
        self.cpu_val: Any = None
        self.ram_val: Any = None
        self.xp_val: Any = None
        self.ai_val: Any = None
        self.main_grid: Any = None
        self.tabs: Any = None
        self._anim_counters: Dict[str, int] = {"cpu": 0, "ram": 0, "xp": 0, "ai": 0}
        self.cpu_icon: Any = None
        self.ram_icon: Any = None
        self.xp_icon: Any = None
        self.ai_icon: Any = None
        
        self._build_ui()
        self._update_loop()

    def _build_ui(self):
        # Header Shard
        head = tk.Frame(self, bg=PAL["background"], height=80, padx=40)
        head.pack(side="top", fill="x", pady=20)
        tk.Label(head, text="MORPHIC DASHBOARD", font=FONT["h1"], fg=PAL["primary"], bg=PAL["background"]).pack(side="left")
        self.time_lbl = tk.Label(head, text="00:00:00", font=("JetBrains Mono", 14), fg=PAL["primary"], bg=PAL["background"])
        self.time_lbl.pack(side="right")

        # Main Performance Grid
        self.main_grid = tk.Frame(self, bg=PAL["background"], padx=40)
        self.main_grid.pack(fill="both", expand=True)

        # Telemetry Shards with Animated Icons
        self.cpu_icon = self._create_card(self.main_grid, "HARDWARE CPU", "cpu_val", 0, 0, icon_key="hal")
        self.ram_icon = self._create_card(self.main_grid, "HARDWARE RAM", "ram_val", 0, 1, icon_key="memory")
        self.xp_icon = self._create_card(self.main_grid, "ACADEMIC XP", "xp_val", 1, 0, icon_key="ncert")
        self.ai_icon = self._create_card(self.main_grid, "AI FLEET STATUS", "ai_val", 1, 1, icon_key="intelligence")

        # Tab: Proximity Launchers
        self.tabs = ttk.Notebook(self.main_grid)
        self.tabs.grid(row=0, column=2, rowspan=2, sticky="nsew", padx=10, pady=10)
        self.main_grid.grid_columnconfigure(2, weight=1)

        if SovereignStrategist:
            ch_fr = tk.Frame(self.tabs, bg=PAL["background"], pady=30)
            self.tabs.add(ch_fr, text=" ♟️ CHESS ")
            tk.Button(ch_fr, text="LAUNCH SOVEREIGN CHESS", bg=PAL["accent"], fg="white", 
                      command=lambda: SovereignStrategist().mainloop()).pack(expand=True) # type: ignore
            
        if LudoApp:
            ld_fr = tk.Frame(self.tabs, bg=PAL["background"], pady=30)
            self.tabs.add(ld_fr, text=f" {ICONS.get('board_hub', '🎲')} LUDO ")
            tk.Button(ld_fr, text="LAUNCH DETERMINISTIC LUDO", bg=PAL["accent"], fg="white", 
                      command=lambda: LudoApp().mainloop()).pack(expand=True) # type: ignore

        # Tool Nexus: Comprehensive Icon-Rich Launcher
        nexus_fr = tk.Frame(self.tabs, bg=PAL["background"], padx=20, pady=20)
        self.tabs.add(nexus_fr, text=f" {ICONS.get('nexus', '📡')} TOOL NEXUS ")
        
        # Grid of Tool Icons
        tools = [
            ("fs", "SigmaFS"), ("perf", "Boost"), ("shield", "AuraShield"), 
            ("crusher", "Crusher"), ("automator", "Automation"), ("portal", "Transparency"),
            ("ghostchat", "GhostChat"), ("ncert", "Virtual Lab"), ("studio", "SigmaStudio")
        ]
        for i, (key, name) in enumerate(tools):
            r, c = divmod(i, 3)
            btn = tk.Button(nexus_fr, text=f"{ICONS.get(key, '🔹')}\n{name}", 
                            font=("Inter", 9, "bold"), bg=PAL["surface"], fg="white", 
                            relief="flat", width=12, height=4, highlightthickness=1, highlightbackground=PAL["border"])
            btn.grid(row=r, column=c, padx=5, pady=5)

        # Bottom Vibe Control
        ctrl = tk.Frame(self, bg=PAL["surface"], height=100, padx=40)
        ctrl.pack(side="bottom", fill="x")
        tk.Label(ctrl, text="SYSTEM VIBE", font=("Inter Bold", 10), fg=PAL["primary"], bg=PAL["surface"]).pack(side="left")
        for v in ["DEEP_SPACE", "APEX_GOLD", "ZEN_FOCUS", "GAMING_NEON"]:
            tk.Button(ctrl, text=v, bg="#1A1D23", fg="white", relief="flat", padx=15, pady=8, 
                      command=lambda v=v: self._switch_vibe(v)).pack(side="left", padx=10) # type: ignore

    def _create_card(self, parent, title, attr, row, col, icon_key="hal"):
        card = tk.Frame(parent, bg=PAL["surface"], padx=30, pady=30, highlightthickness=1, highlightbackground="#2A2D35")
        card.grid(row=row, column=col, sticky="nsew", padx=10, pady=10)
        parent.grid_columnconfigure(col, weight=1)
        parent.grid_rowconfigure(row, weight=1)
        
        icon_fr = tk.Frame(card, bg=PAL["surface"])
        icon_fr.pack(anchor="w", fill="x")
        
        icon_lbl = tk.Label(icon_fr, text=ICONS.get(icon_key, "🔹"), font=("Inter Bold", 14), fg=PAL["primary"], bg=PAL["surface"])
        icon_lbl.pack(side="left")
        
        tk.Label(icon_fr, text=f" {title}", font=("Inter Bold", 10), fg="#8E8E93", bg=PAL["surface"]).pack(side="left")
        val = tk.Label(card, text="--", font=("Inter Bold", 36), fg="white", bg=PAL["surface"])
        val.pack(anchor="w", pady=(10, 0))
        setattr(self, attr, val)
        return icon_lbl

    def _switch_vibe(self, vibe):
        if self.kernel and hasattr(self.kernel, "bus"):
            self.kernel.bus.emit("governor.vibe_switch", {"vibe": vibe})
        messagebox.showinfo("Vibe Switch", f"System Atmosphere updated to: {vibe}")

    def _update_loop(self):
        if self.time_lbl:
            self.time_lbl.config(text=time.strftime("%H:%M:%S"))
        
        # Hydrate from Shards with absolute safety checks
        hal = self.hal
        if hal and hasattr(hal, "get_hardware_state"):
            state = hal.get_hardware_state()
            if state and self.cpu_val:
                self.cpu_val.config(text=f"{state.get('cpu', 0)}%")
            if state and self.ram_val:
                self.ram_val.config(text=f"{state.get('ram', 0)}%")
        
        if self.edu and hasattr(self.edu, "xp") and self.xp_val:
            self.xp_val.config(text=str(getattr(self.edu, "xp", 0)))
            
        if self.ai and hasattr(self.ai, "platforms") and self.ai_val:
            platforms = getattr(self.ai, "platforms", [])
            nodes = len(platforms) if platforms else 0
            self.ai_val.config(text=f"{nodes} NODES")

        # Animate Icons
        if self.cpu_icon:
            self.cpu_icon.config(text=SPINNERS["pulse"][self._anim_counters["cpu"] % len(SPINNERS["pulse"])])
            self._anim_counters["cpu"] += 1
        if self.ram_icon:
            self.ram_icon.config(text=SPINNERS["gear"][self._anim_counters["ram"] % len(SPINNERS["gear"])])
            self._anim_counters["ram"] += 1
        if self.xp_icon:
            self.xp_icon.config(text=SPINNERS["orbit"][self._anim_counters["xp"] % len(SPINNERS["orbit"])])
            self._anim_counters["xp"] += 1
        if self.ai_icon:
            self.ai_icon.config(text=SPINNERS["neural"][self._anim_counters["ai"] % len(SPINNERS["neural"])])
            self._anim_counters["ai"] += 1

        self.after(500, self._update_loop)

if __name__ == "__main__":
    MorphicDashboard().mainloop()

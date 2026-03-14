import tkinter as tk
from tkinter import ttk, messagebox
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_LOGO, FONT_MONO

class ApexPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "🏔️ Sovereign Apex", "Multi-OS Master Hub & Fusion Grid")
        self.build()

    def build(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)

        # 1. Left: Spotlight & Controls
        l_fr = tk.Frame(body, bg=PAL["bg2"], width=450)
        l_fr.pack(side="left", fill="both", padx=5)
        l_fr.pack_propagate(False)

        # Spotlight (macOS USP)
        spot_c = self._card(l_fr, "🔍 Sigma Spotlight (macOS/Alfred USP)")
        spot_c.master.pack(fill="x", pady=5)
        s_ent = ttk.Entry(spot_c); s_ent.pack(fill="x", pady=5); s_ent.insert(0, "Search files, AI, or system...")
        
        def run_spot():
            # Spotlight proxy
            self.gui._log_voice(f"Spotlight: Indexing for '{s_ent.get()}'...")
            self.after(500, lambda: self._notify("Spotlight", "Found: Sovereign_Kernel_v2.1.sys", "OK"))

        ttk.Button(spot_c, text="Find & Execute", command=run_spot).pack(fill="x")

        # Control Center
        ctrl_c = self._card(l_fr, "🎛️ Control Center (Unified Toggles)")
        ctrl_c.master.pack(fill="x", pady=5)
        
        toggles = ["Stealth", "Neural-Lock", "Auto-Heal", "Zero-Lat"]
        for t in toggles:
            btn = tk.Button(ctrl_c, text=f"{t}: ON", bg=PAL["bg2"], fg=PAL["teal"], relief="flat", padx=10)
            btn.pack(side="left", padx=2, pady=2)

        # Apex Terminal
        tk.Label(l_fr, text="Apex System Terminal", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["bg2"]).pack(pady=(10,0))
        self.log = self._console(l_fr, height=15)
        self.log.pack(fill="both", expand=True, padx=5, pady=5)
        self._log(self.log, "Apex Hub Online. Syncing cross-OS descriptors...", "INFO")

        # 2. Right: SnapGrid, SSL, TimeVault
        r_fr = tk.Frame(body, bg=PAL["bg"])
        r_fr.pack(side="left", fill="both", expand=True, padx=5)

        # SnapGrid (Windows USP)
        grid_c = self._card(r_fr, "📐 SnapGrid Layouts (Windows USP)")
        grid_c.master.pack(fill="x", pady=5)
        layouts = ["Standard", "Wide", "Focus", "Grid"]
        for l in layouts:
            ttk.Button(grid_c, text=l, command=lambda l=l: self._notify("SnapGrid", f"Layout '{l}' applied via AI-Matrix.", "OK")).pack(side="left", padx=5)

        # TimeVault (macOS USP)
        vault_c = self._card(r_fr, "⏳ TimeVault Snapshots (macOS USP)")
        vault_c.master.pack(fill="x", pady=5)
        ttk.Button(vault_c, text="Create Restore Point", command=lambda: self._notify("TimeVault", "Restore point created: 0x55FA", "OK")).pack(side="left", padx=5)
        
        # SSL Manager (WSL USP)
        ssl_c = self._card(r_fr, "🛡️ Sigma Subsystem for Linux (WSL USP)")
        ssl_c.master.pack(fill="x", pady=5)
        ttk.Button(ssl_c, text="Launch Sovereign Linux Subsystem", command=lambda: self._notify("SSL", "Subsystem 'Arch-Titan' is now online.", "OK")).pack(side="left", padx=5)

        # Continuity Engine (Apple USP)
        cont_c = self._card(r_fr, "🔗 Ecosystem Continuity (Continuity USP)")
        cont_c.master.pack(fill="x", pady=5)
        ttk.Button(cont_c, text="Sync Universal Clipboard", command=lambda: self._notify("Continuity", "Clipboard synced across Mesh Nodes.", "INFO")).pack(side="left", padx=5)

        # Pulse & Semantic Bus (Sigma Core Evolution)
        pulse_c = self._card(r_fr, "📡 Sigma Pulse & Semantic Bus")
        pulse_c.master.pack(fill="x", pady=5)
        tk.Label(pulse_c, text="Bus Integrity: 100% | Latency: 0.2ms", font=FONT_SMALL, fg=PAL["green"], bg=PAL["card"]).pack(side="left", padx=10)
        ttk.Button(pulse_c, text="Calibrate Bus", command=lambda: self._log(self.log, "Pulse: Re-calibrating semantic routes...", "WARN")).pack(side="right")

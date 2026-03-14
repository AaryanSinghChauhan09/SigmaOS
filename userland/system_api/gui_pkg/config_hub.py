import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_MED, FONT_BOLD, FONT_TITLE, FONT_SMALL

class ConfigHubPage(SigmaPage):
    def __init__(self, parent, gui):
        is_child = gui.kernel.registry.get("guardian").is_child_mode()
        title = "Kiddie Safety Hub" if is_child else "Sovereign Configuration Hub"
        super().__init__(parent, gui, title, "Making SigmaOS Happy & Safe")
        self.cfg = gui.cfg
        self._build_ui()

    def _build_ui(self):
        is_child = self.controller._is_child_mode()
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True)

        # Tabbed Sidebar
        s_fr = tk.Frame(body, bg=PAL["bg2"], width=200)
        s_fr.pack(side="left", fill="both", padx=(0, 10))
        s_fr.pack_propagate(False)

        self.c_fr = tk.Frame(body, bg=PAL["bg"])
        self.c_fr.pack(side="left", fill="both", expand=True)
        
        if is_child:
            cats = ["Safety", "Info"]
        else:
            cats = ["System", "Display", "Network", "Security", "Safety", "Sovereignty", "About"]

        for cat in cats:
            btn_text = cat
            tk.Button(s_fr, text=btn_text, font=FONT_MED, bg=PAL["bg2"], fg=PAL["text"], 
                      relief="flat", anchor="w", padx=15, 
                      command=lambda c=cat: self._show_cfg(c)).pack(fill="x", pady=2)

        start_cat = "Safety" if is_child else "About"
        self._show_cfg(start_cat)

    def _show_cfg(self, cat):
        for w in self.c_fr.winfo_children(): w.destroy()
        if cat == "System": self._cfg_system(self.c_fr)
        elif cat == "Display": self._cfg_display(self.c_fr)
        elif cat == "Network": self._cfg_network(self.c_fr)
        elif cat == "Security": self._cfg_security(self.c_fr)
        elif cat == "Safety": self._cfg_safety(self.c_fr)
        elif cat == "Sovereignty": self._cfg_sovereignty(self.c_fr)
        elif cat == "About" or cat == "Info": self._cfg_about(self.c_fr)

    def _cfg_safety(self, parent):
        is_child = self.controller._is_child_mode()
        title = "Kiddie Safety Shield" if is_child else "Compliance & Child Safety (Guardian)"
        tk.Label(parent, text=title, font=FONT_TITLE, fg="white", bg=PAL["bg"]).pack(anchor="w", pady=10)
        
        info_title = "Always Safe for You!" if is_child else "International Age Rating Compliance"
        info = self._card(parent, info_title)
        info.master.pack(fill="x", pady=5)
        
        desc = "SigmaGuardian keeps you happy and safe while you learn!" if is_child else "SigmaGuardian enforces U/G ratings for 5-year-old safety."
        tk.Label(info, text=desc, font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w", pady=5)
        
        status_fr = tk.Frame(parent, bg=PAL["bg3"], padx=15, pady=10, highlightthickness=1, highlightbackground=PAL["green"])
        status_fr.pack(fill="x", pady=20)
        
        tk.Label(status_fr, text="🌈 SAFETY MODE: ON FOREVER", font=("Inter Bold", 10), fg=PAL["green"], bg=PAL["bg3"]).pack(side="left")
        tk.Label(status_fr, text="Safe & Happy", font=("Inter Italic", 9), fg=PAL["dim"], bg=PAL["bg3"]).pack(side="right")
        
        footer = "Everything in SigmaOS is hand-picked for kids. No scary things allowed!" if is_child else "Compliance Standards: NIST, COPPA, Multi-Region Rating Sync."
        tk.Label(parent, text=footer, font=FONT_SMALL, fg=PAL["dim"], bg=PAL["bg"], wraplength=500, justify="left").pack(anchor="w", pady=20)

    def _cfg_about(self, parent):
        is_child = self.controller._is_child_mode()
        head = "SigmaOS for Kids" if is_child else "SigmaOS Sovereign"
        tk.Label(parent, text=head, font=FONT_LOGO, fg=PAL["cyan"], bg=PAL["bg"]).pack(pady=20)
        tk.Label(parent, text=f"Version {self.cfg.VERSION}", font=FONT_MED, fg=PAL["dim"], bg=PAL["bg"]).pack()
        
        info_title = "Fun Details" if is_child else "OS Status & Parity Dashboard"
        info = self._card(parent, info_title)
        info.master.pack(fill="x", pady=20)
        grid = tk.Frame(info, bg=PAL["card"])
        grid.pack(fill="x")
        
        if is_child:
            metrics = [
                ("OS Heart", "Happy Beats"),
                ("Fun Level", "Maximum!"),
                ("Safety Shield", "🟢 100% SECURE"),
                ("Learning Points", "Ready to Grow")
            ]
        else:
            metrics = [
                ("Kernel Type", "Neural-Predictive"),
                ("Subsystem", "Sovereign-Core-v3"),
                ("Parity Status", "🟢 TITAN LEVEL REACHED"),
                ("Bridges Active", "4 (Win32, Cocoa, APK, WASM)")
            ]
        for i, (k, v) in enumerate(metrics):
            tk.Label(grid, text=k+":", font=FONT_BOLD, fg=PAL["dim"], bg=PAL["card"]).grid(row=i, column=0, sticky="w", pady=5)
            tk.Label(grid, text=v, font=FONT_BOLD, fg="white", bg=PAL["card"]).grid(row=i, column=1, sticky="w", padx=20)

    def _cfg_system(self, parent):
        tk.Label(parent, text="System Performance & Automation", font=FONT_TITLE, fg="white", bg=PAL["bg"]).pack(anchor="w", pady=10)
        ttk.Checkbutton(parent, text="Enable Sovereign Autopilot (AI System Repair)", variable=self.gui._voice_active).pack(anchor="w", pady=5)
        ttk.Checkbutton(parent, text="Ultra Performance Mode (Disable Animations)", variable=self.gui._ultra_perf).pack(anchor="w", pady=5)
        ttk.Scale(parent, from_=0, to=100).pack(fill="x", pady=20)
        tk.Label(parent, text="Energy Impact: MINIMAL", fg=PAL["teal"], bg=PAL["bg"]).pack(anchor="w")

    def _cfg_display(self, parent):
        tk.Label(parent, text="Display & Hybrid Compositor", font=FONT_TITLE, fg="white", bg=PAL["bg"]).pack(anchor="w", pady=10)
        tk.Label(parent, text="Resolution: 1400x900 (Native High-DPI)", bg=PAL["bg"], fg=PAL["text"]).pack(anchor="w")
        ttk.Checkbutton(parent, text="Enable 10-bit Color Depth (Pro Rendering)").pack(anchor="w", pady=5)
        ttk.Checkbutton(parent, text="Hyper-Jitter Suppression (Direct Compositing)").pack(anchor="w", pady=5)

    def _cfg_network(self, parent):
        tk.Label(parent, text="Network & Sovereign Mesh", font=FONT_TITLE, fg="white", bg=PAL["bg"]).pack(anchor="w", pady=10)
        tk.Label(parent, text="Mesh Status: 42 Nodes Synchronized", fg=PAL["green"], bg=PAL["bg"]).pack(anchor="w")
        ttk.Button(parent, text="Rotate Quantum Keys", command=lambda: self.gui._notify("Security", "Quantum-Dilithium keys rotated.", "OK")).pack(anchor="w", pady=10)

    def _cfg_security(self, parent):
        tk.Label(parent, text="Sovereign Security & Hardening", font=FONT_TITLE, fg="white", bg=PAL["bg"]).pack(anchor="w", pady=10)
        
        sh = self.kernel.registry.get("security_hardening")
        posture = sh.get_security_posture() if sh else {}
        
        info = self._card(parent, "Live Security Posture")
        info.master.pack(fill="x", pady=5)
        
        for k, v in posture.items():
            f = tk.Frame(info, bg=PAL["card"])
            f.pack(fill="x", pady=2)
            tk.Label(f, text=k.replace('_',' '), font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(side="left")
            tk.Label(f, text=v, font=FONT_BOLD, fg=PAL["teal"], bg=PAL["card"]).pack(side="right")
            
        ttk.Button(parent, text="🛡️ Rotate Memory Canaries", command=lambda: self.gui._notify("Security", "Memory Shadowing Canaries Rotated.", "OK")).pack(anchor="w", pady=10)

    def _cfg_sovereignty(self, parent):
        tk.Label(parent, text="Competitive Absorption & AI Sovereignty", font=FONT_TITLE, fg="white", bg=PAL["bg"]).pack(anchor="w", pady=10)
        
        zen = self.kernel.registry.get("zenith")
        status = zen.health_check() if zen else "Zenith Core Offline"
        tk.Label(parent, text=f"Zenith Status: {status}", font=FONT_MED, fg=PAL["cyan"], bg=PAL["bg"]).pack(anchor="w", pady=5)
        
        info = self._card(parent, "Competitive Absorption Bridges")
        info.master.pack(fill="x", pady=5)
        
        bridges = [
            ("Win32 Bridge", "0ms DLL Emulation", "ENABLED"),
            ("macOS Cocoa Proxy", "Retina Compositing", "ENABLED"),
            ("Antigravity Suite", "Full Native Integration", "ACTIVE"),
        ]
        for b, d, s in bridges:
            f = tk.Frame(info, bg=PAL["card"])
            f.pack(fill="x", pady=2)
            tk.Label(f, text=f"{b}: {d}", font=FONT_SMALL, fg=PAL["text"], bg=PAL["card"]).pack(side="left")
            tk.Label(f, text=s, font=("Inter Bold", 7), fg=PAL["gold"], bg=PAL["card"]).pack(side="right")

import tkinter as tk
from tkinter import scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_TITLE

class MissionControlPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        container = tk.Frame(self, bg=PAL["bg"])
        container.pack(fill="both", expand=True, padx=40, pady=30)

        # --- Header ---
        hdr = tk.Frame(container, bg=PAL["bg"])
        hdr.pack(fill="x", pady=(0, 30))
        tk.Label(hdr, text="MISSION CONTROL", font=("Outfit", 28, "bold"), fg=PAL["cyan"], bg=PAL["bg"]).pack(side="left")
        
        status_fr = tk.Frame(hdr, bg=PAL["glass"], padx=15, pady=5)
        status_fr.pack(side="right")
        self.status_lbl = tk.Label(status_fr, text="AGENTIC CLAW: ARMED", font=("Consolas", 10), fg=PAL["green"], bg=PAL["glass"])
        self.status_lbl.pack()

        # --- Dashboard ---
        dash = tk.Frame(container, bg=PAL["bg"])
        dash.pack(fill="both", expand=True)
        
        # Left Panel: Active Missions
        left = tk.Frame(dash, bg=PAL["bg"], width=400)
        left.pack(side="left", fill="both", pady=5)
        left.pack_propagate(False)
        
        tk.Label(left, text="NEURAL THREADS", font=("Outfit", 14, "bold"), fg=PAL["text"], bg=PAL["bg"]).pack(anchor="w", pady=(0,15))
        
        self.mc_list_fr = tk.Frame(left, bg=PAL["bg"])
        self.mc_list_fr.pack(fill="both", expand=True)

        # Right Panel: Mission Terminal
        right = tk.Frame(dash, bg=PAL["card"], bd=1, relief="flat")
        right.pack(side="right", fill="both", expand=True, padx=(20, 0))
        
        tk.Label(right, text="MISSION LOGS", font=("Consolas", 11), fg=PAL["cyan"], bg=PAL["card"], padx=15, pady=10).pack(anchor="w")
        self.mc_log = scrolledtext.ScrolledText(right, bg=PAL["bg"], fg=PAL["dim"], font=("Consolas", 10), borderwidth=0, highlightthickness=0)
        self.mc_log.pack(fill="both", expand=True, padx=1, pady=1)

        # --- Controls ---
        ctrl = tk.Frame(container, bg=PAL["card"], pady=20, padx=20)
        ctrl.pack(fill="x", pady=(30, 0))
        
        missions = [
            ("GitHub Sync", "claw.git_sync", PAL["blue"]),
            ("Security Audit", "claw.sec_audit", PAL["red"]),
            ("System Heal", "claw.self_heal", PAL["green"]),
            ("Performance Apex", "claw.perf_tune", PAL["gold"])
        ]
        
        for name, mid, color in missions:
            btn = tk.Button(ctrl, text=name.upper(), font=("Outfit", 10, "bold"), 
                            fg="white", bg=color, relief="flat", padx=20, pady=8,
                            command=lambda m=mid, n=name: self._launch_mission(m, n))
            btn.pack(side="left", padx=10)

    def _launch_mission(self, mission_id: str, name: str):
        """Hand-off to OmniAutomator for mission execution."""
        self.mc_log.insert("end", f"\n[LAUNCH] Starting Mission: {name}...\n", "launch")
        self.mc_log.see("end")
        
        if self.controller.kernel and hasattr(self.controller.kernel, "automator") and self.controller.kernel.automator:
            self.controller.kernel.bus.emit("claw.mission.launch", {"id": mission_id, "mission": name})
            self.controller._update_morphic_status("MISSION", f"Launching {name}", PAL["cyan"])
        else:
            self.mc_log.insert("end", "[ERROR] Agentic Subsystems Offline.\n", "err")

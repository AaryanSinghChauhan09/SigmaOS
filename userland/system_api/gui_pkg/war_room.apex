import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL

class WarRoomPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "COMPETITOR WAR ROOM", "Live Performance Superiority vs. n8n, OpenClaw, Windows & macOS")

        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)
        
        # Dominance Stats (Top Bar)
        stat_bar = tk.Frame(body, bg=PAL["bg2"])
        stat_bar.pack(fill="x", pady=(0, 20))
        
        intel = self.controller.kernel.intel
        report = intel.run_benchmark()
        
        for i, (label, val) in enumerate([("Total Wins", report["wins"]), ("Dominance", report["dominance"]), ("Status", "GLOBAL LEAD")]):
            f = tk.Frame(stat_bar, bg=PAL["bg2"], padx=20, pady=10)
            f.pack(side="left", expand=True)
            tk.Label(f, text=label, font=FONT_SMALL, fg=PAL["dim"], bg=PAL["bg2"]).pack()
            tk.Label(f, text=str(val), font=("Segoe UI", 16, "bold"), fg=PAL["gold"], bg=PAL["bg2"]).pack()

        # Comparison Grid
        grid = tk.Frame(body, bg=PAL["bg"])
        grid.pack(fill="both", expand=True)
        
        comps = ["Windows 11", "macOS Sequoia", "n8n (Workflow)", "OpenClaw (Agent)"]
        for i, comp in enumerate(comps):
            r, c = divmod(i, 2)
            card = self.controller._card(grid, f"Target: {comp}")
            card.master.grid(row=r, column=c, padx=10, pady=10, sticky="nsew")
            
            deltas = intel.get_live_delta(comp)
            # Show top 3 deltas
            for d in deltas[:4]:
                f = tk.Frame(card, bg=PAL["card"])
                f.pack(fill="x", pady=2)
                tk.Label(f, text=d["metric"], font=FONT_SMALL, fg=PAL["text"], bg=PAL["card"]).pack(side="left")
                tk.Label(f, text=f"Sigma: {d['sigma']} | {comp}: {d['competitor']}", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(side="left", padx=10)
                tk.Label(f, text=d["advantage"], font=FONT_SMALL, fg=PAL["green"], bg=PAL["card"]).pack(side="right")
                
            tk.Label(card, text="-"*40, fg=PAL["dim"], bg=PAL["card"]).pack()
            tk.Label(card, text="USP CRUSH:", font=FONT_SMALL, fg=PAL["gold"], bg=PAL["card"]).pack(anchor="w")
            
            crush_msg = "Dominating kernel throughput."
            if "n8n" in comp: crush_msg = "2s Auto-Plan vs 120s Manual Node Sync."
            if "OpenClaw" in comp: crush_msg = "Ring-0 Token Guard vs Plaintext Key Memory."
            if "Windows" in comp: crush_msg = "290MB Idle vs 4.2GB Blob."
            
            tk.Label(card, text=crush_msg, font=FONT_SMALL, fg=PAL["text"], bg=PAL["card"]).pack(anchor="w")

        grid.grid_columnconfigure(0, weight=1)
        grid.grid_columnconfigure(1, weight=1)

        def _refresh():
            self.controller._show_page("war_room") # Simple re-build
        ttk.Button(body, text="🔄 RE-RUN LIVE BENCHMARKS", command=_refresh).pack(pady=10)

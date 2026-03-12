import tkinter as tk
from tkinter import ttk, scrolledtext
import random
import time
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MONO, FONT_MED, FONT_TITLE

class DashboardPage(SigmaPage):
    def __init__(self, parent, gui):
        super().__init__(parent, gui)
        self.build()

    def build(self):
        # Header is already handled by SigmaGUI if we want, but base_page doesn't do it automatically.
        # Actually SigmaGUI calls _build_page_header.
        # But for Dashboard we might want a custom look.
        
        container = tk.Frame(self, bg=PAL["bg"])
        container.pack(fill="both", expand=True, padx=20, pady=10)

        # 1. Headline Stats Row
        stats_row = tk.Frame(container, bg=PAL["bg"])
        stats_row.pack(fill="x", pady=(0, 20))

        stat_defs = [
            ("ram", "RAM UTILIZATION", "12%", PAL["cyan"]),
            ("cpu", "CPU CORE LOAD", "4%", PAL["teal"]),
            ("cap", "SYSTEM CAPACITY", "MAX", PAL["gold"]),
            ("events", "KERNEL EVENTS", "0", PAL["accent"]),
            ("sec", "SECURITY SCORE", "100", PAL["green"]),
        ]

        for key, label, val, color in stat_defs:
            var = tk.StringVar(value=val)
            self.controller._stat_widgets[key] = var
            card = self.controller._card(stats_row, label)
            card.master.pack(side="left", fill="both", expand=True, padx=5)
            
            tk.Label(card, textvariable=var, font=("Inter Bold", 20), fg=color, bg=PAL["card"]).pack(anchor="w")
            
            pb_fr = tk.Frame(card, bg=PAL["border"], height=4)
            pb_fr.pack(fill="x", pady=(10, 0))
            inner_pb = tk.Frame(pb_fr, bg=color, width=40, height=4)
            inner_pb.place(x=0, y=0)
            
            # Map for live updates
            if key == "ram": self.controller._ram_pb = inner_pb
            if key == "cpu": self.controller._cpu_pb = inner_pb

        # 2. Hero Component: Live Heatmap & Crusher
        hero_fr = tk.Frame(container, bg=PAL["bg"])
        hero_fr.pack(fill="x", pady=(0, 20))

        # Heatmap Canvas
        self.heatmap_fr = self.controller._card(hero_fr, "⚡ REAL-TIME KERNEL HEATMAP")
        self.heatmap_fr.master.pack(side="left", fill="both", expand=True, padx=(0, 10))
        
        self.heatmap_canvas = tk.Canvas(self.heatmap_fr, height=40, bg=PAL["bg3"], highlightthickness=0)
        self.heatmap_canvas.pack(fill="x", pady=5)
        self.controller._heatmap_canvas = self.heatmap_canvas
        self._draw_heatmap()

        # Crusher Card
        cr_card = self.controller._card(hero_fr, "🛡️ COMPETITOR CRUSHER")
        cr_card.master.pack(side="left", fill="both", expand=True)
        
        crusher = self.controller.kernel.registry.get("crusher")
        curr_cr = tk.Label(cr_card, text=crusher.health_check() if crusher else "Crusher Offline", 
                           bg=PAL["card"], fg=PAL["gold"], font=FONT_BOLD)
        curr_cr.pack(side="left", padx=10)
        
        def _defeat():
            msg = crusher.defeat_telemetry() if crusher else "N/A"
            self.controller._notify("Crusher", msg, "OK")

        ttk.Button(cr_card, text="Defeat Telemetry", command=_defeat).pack(side="right", padx=10)

        # 3. Main Body: Commands (Left), Telemetry (Center), Blame (Right)
        body = tk.Frame(container, bg=PAL["bg"])
        body.pack(fill="both", expand=True)

        # Left: Quick Commands
        qa = self.controller._card(body, "GLOBAL COMMANDS")
        qa.master.pack(side="left", fill="both", expand=False, padx=(0, 10))
        qa.master.configure(width=220)
        qa.master.pack_propagate(False)

        def _btn(lbl, cmd, color=PAL["accent"]):
             b = tk.Button(qa, text=lbl, command=cmd, font=FONT_SMALL,
                           bg=PAL["bg3"], fg=PAL["text"], activebackground=color,
                           relief="flat", bd=0, pady=8)
             b.pack(fill="x", pady=4)
             b.bind("<Enter>", lambda e: b.config(bg=color))
             b.bind("<Leave>", lambda e: b.config(bg=PAL["bg3"]))

        _btn("Run Boot Sequence", self.controller._do_boot, PAL["blue"])
        _btn("Aether Sync", lambda: self.controller._show_page("aether_orch"), PAL["purple"])
        _btn("Security Audit", lambda: self.controller._show_page("war_room"), PAL["red"])
        _btn("Performance Tune", self.controller._do_health, PAL["teal"])
        _btn("Mission Control", self.controller._show_mission_control, PAL["accent"])

        # Center: Telemetry Log
        log_card = self.controller._card(body, "SOVEREIGN EVENT TELEMETRY")
        log_card.master.pack(side="left", fill="both", expand=True)
        
        self.dash_log = self.controller._console(log_card, height=20)
        self.dash_log.pack(fill="both", expand=True, pady=(0, 10))
        self.controller._dash_log = self.dash_log
        self.controller._log(self.dash_log, "Dashboard Online. Monitoring Kernel Bus...", "HEAD")

        # Right: Competitor Blame
        met_card = self.controller._card(body, "COMPETITOR BLAME")
        met_card.master.pack(side="left", fill="both", expand=False, padx=(10, 0))
        met_card.master.configure(width=280)
        met_card.master.pack_propagate(False)

        self.blame_scroll = tk.Frame(met_card, bg=PAL["card"])
        self.blame_scroll.pack(fill="both", expand=True, pady=5)
        self._refresh_blame()

        def _purge():
             self.controller._log(self.dash_log, "⚡ PURGING SHIM DEBT: Cycle reclamation engaged.", "WARN")
             if hasattr(self.controller.kernel, "perf"):
                  res = self.controller.kernel.perf.steal_cycle_from_shims()
                  self.controller._log(self.dash_log, f"✔ Reclaimed {res['reclaimed_tflops']} TFLOPS.", "OK")
             self._refresh_blame()

        tk.Button(met_card, text="PURGE SHIMS", font=("Inter Bold", 8), bg=PAL["red"], fg="white", bd=0, command=_purge).pack(side="bottom", fill="x", pady=5)

    def _draw_heatmap(self):
        if not self.heatmap_canvas.winfo_exists(): return
        self.heatmap_canvas.delete("all")
        W = self.heatmap_canvas.winfo_width()
        if W < 10: W = 200
        
        for i in range(12):
            x1 = (W/12) * i
            x2 = x1 + (W/12) - 3
            intensity = random.randint(40, 120)
            color = f"#{intensity:02x}20{200-intensity:02x}"
            if i % 4 == 0: color = PAL["accent"] if random.random() > 0.7 else PAL["bg3"]
            self.heatmap_canvas.create_rectangle(x1, 5, x2, 35, fill=color, outline="")
        
        self.after(1000, self._draw_heatmap)

    def _refresh_blame(self):
        for child in self.blame_scroll.winfo_children(): child.destroy()
        if hasattr(self.controller.kernel, "perf"):
            blame_list = self.controller.kernel.perf.get_competitor_blame()
            if not blame_list:
                tk.Label(self.blame_scroll, text="✔ No Shims Detected.", font=FONT_SMALL, fg=PAL["green"], bg=PAL["card"]).pack()
            else:
                for b in blame_list:
                     f = tk.Frame(self.blame_scroll, bg=PAL["card"])
                     f.pack(fill="x", pady=2)
                     tk.Label(f, text=f"✖ {b['name']}", font=FONT_MONO, fg=PAL["dim"], bg=PAL["card"]).pack(side="left")
                     tk.Label(f, text=b['usage'], font=FONT_MONO, fg=PAL["red"], bg=PAL["card"]).pack(side="right")
        self.after(5000, self._refresh_blame)

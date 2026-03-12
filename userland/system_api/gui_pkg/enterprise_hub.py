import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD

class EnterpriseHubPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "ANTIGRAVITY ENTERPRISE HUB", "Unified Access to Sovereign Productivity Suite")
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        # Tools Grid
        tools = [
            ("\ud83d\udcc1 Tools Finder", "ag_finder", "sigma.sys.ag_finder"),
            ("\ud83d\udce7 Email Discovery", "email_disco", "sigma.ai.email_disco"),
            ("\ud83d\udcca Excel AI Filler", "excel_ai", "sigma.prod.excel_ai"),
            ("\ud83e\uddea Excel Preproc", "excel_preproc", "sigma.prod.excel_preproc"),
            ("\ud83d\udcd1 PDF Forge", "pdf_forge", "sigma.prod.pdf_forge"),
            ("\ud83d\udd21 Pure Text", "pure_text", "sigma.prod.pure_text"),
            ("\u2728 Text Cleaner", "text_cleaner", "sigma.prod.text_cleaner"),
            ("\ud83d\udcfd Titan Capture", "titan_capture", "sigma.sys.titan_capture"),
            ("\ud83d\uddec IndentFlow", "indent_flow", "sigma.dev.indent_flow"),
            ("\ud83e\ude90 AG Zenith", "ag_zenith", "sigma.ai.ag_zenith"),
            ("\ud83c\udf10 Mesh Monitor", "mesh_monitor", "sigma.ai.mesh_monitor"),
            ("\u26a1 Sovereign De-bloater", "debloater", "sigma.sys.debloater"),
            ("\ud83c\udf2a\ufe0f AG Shuffler", "shuffler", "sigma.sys.shuffler"),
            ("\ud83d\udcd6 Software Guide", "guide", "sigma.doc.ag_guide"),
            ("\ud83d\udccb Scrum Board", "scrum", "sigma.prod.scrum"),
            ("\ud83d\udcca Gantt Chart", "gantt", "sigma.prod.gantt"),
            ("\u23f2\ufe0f Time Tracker", "tracker", "sigma.prod.tracker"),
        ]

        def _debloat():
            if hasattr(self.controller.kernel, 'ag_ent'):
                res = self.controller.kernel.ag_ent.debloater.perform_debloat()
                self.controller._notify("Performance", res, "OK")
                self.controller._ultra_perf.set(True)

        def _shuffle():
            if hasattr(self.controller.kernel, 'ag_ent'):
                import os
                home_path = os.environ.get("USERPROFILE", "C:/")
                res = self.controller.kernel.ag_ent.shuffler.shuffle_organize(os.path.join(home_path, "Desktop"))
                self.controller._notify("Antigravity Shuffler", res, "OK")

        def _sync_scrum():
            if hasattr(self.controller.kernel, 'ag_ent'):
                msg = self.controller.kernel.ag_ent.scrum.add_task("Initial OS Optimization", "High")
                self.controller._notify("Project Management", msg, "OK")

        btn_fr = tk.Frame(main_panel, bg=PAL["bg"])
        btn_fr.pack(fill="x", pady=5)
        ttk.Button(btn_fr, text="\ud83d\ude80 SOVEREIGN DE-BLOAT", command=_debloat).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="\ud83c\udf2a\ufe0f SHUFFLE WORKSPACE", command=_shuffle).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="\ud83d\udccb SYNC SCRUM", command=_sync_scrum).pack(side="left", padx=5)
        
        grid_fr = tk.Frame(main_panel, bg=PAL["bg"])
        grid_fr.pack(fill="both", expand=True)
        
        for i, (name, lid, app_id) in enumerate(tools):
            row, col = i // 3, i % 3
            card = tk.Frame(grid_fr, bg=PAL["card"], padx=10, pady=10, width=280, height=120)
            card.grid(row=row, column=col, padx=10, pady=10)
            card.pack_propagate(False)
            
            tk.Label(card, text=name, font=FONT_BOLD, fg=PAL["text"], bg=PAL["card"]).pack(anchor="w")
            
            def _launch(aid=app_id):
                self.controller._notify("Enterprise Hub", f"Hydrating {aid}... Running in verified partition.", "OK")
                if hasattr(self.controller, '_launch_app'):
                    self.controller._launch_app(aid)
                elif "pdf_forge" in aid:
                    self.controller._generate_demo_pdf()
            
            ttk.Button(card, text="Launch Tool", command=_launch).pack(side="bottom", fill="x")

        ttk.Button(main_panel, text="\ud83d\udcd6 Open Antigravity Software Guide", command=lambda: self.controller._show_page("ag_guide")).pack(pady=20)

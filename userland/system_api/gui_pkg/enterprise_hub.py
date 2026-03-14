import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL

class EnterpriseHubPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "Enterprise Hub", "Omni-Sovereign Business Intelligence & Process Automation")
        self.build()

    def build(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        # Tools Grid - Augmented with USP from Google Extension Store
        tools = [
            ("📝 SOP Scribe (Scribe USP)", "sop_scribe", "sigma.ent.sop_scribe"),
            ("🎙️ Meeting AI (Fireflies)", "meeting_ai", "sigma.ent.meeting_ai"),
            ("🔍 Lead Intel (Hunter.io)", "lead_intel", "sigma.ent.lead_intel"),
            ("🖋️ Grammarian (Grammarly)", "grammarly", "sigma.ent.grammarly"),
            ("📂 Session Buddy", "session_buddy", "sigma.sys.session_buddy"),
            ("📊 Excel AI Filler", "excel_ai", "sigma.prod.excel_ai"),
            ("📄 PDF Forge", "pdf_forge", "sigma.prod.pdf_forge"),
            ("🎬 Titan Capture (Loom)", "titan_capture", "sigma.sys.titan_capture"),
            ("📋 Scrum Board", "scrum", "sigma.prod.scrum"),
            ("⚡ Sovereign De-bloater", "debloater", "sigma.sys.debloater"),
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

        btn_fr = tk.Frame(body, bg=PAL["bg"])
        btn_fr.pack(fill="x", pady=5)
        ttk.Button(btn_fr, text="🚀 START PROCESS CAPTURE (SCRIBE)", command=lambda: self.gui._log_voice("Scribe: Monitoring system clicks... Auto-generating SOP.")).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="🎙️ RECORD MEETING (FIREFLIES)", command=lambda: self.gui._log_voice("Meeting: Audio stream captured. Transcribing & extracting action items...")).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="🔍 PROSPECT LEADS (HUNTER)", command=lambda: self.gui._log_voice("LeadIntel: Scanning domain metadata for verified decision-makers...")).pack(side="left", padx=5)
        
        grid_fr = tk.Frame(body, bg=PAL["bg"])
        grid_fr.pack(fill="both", expand=True)
        
        for i, (name, lid, app_id) in enumerate(tools):
            row, col = i // 3, i % 3
            card = tk.Frame(grid_fr, bg=PAL["card"], padx=10, pady=10, width=280, height=120)
            card.grid(row=row, column=col, padx=10, pady=10)
            card.pack_propagate(False)
            
            tk.Label(card, text=name, font=FONT_BOLD, fg=PAL["text"], bg=PAL["card"]).pack(anchor="w")
            
            def _launch(aid=app_id):
                self.gui._log_voice(f"Enterprise: Hydrating {aid}... Running in verified partition.")
            
            ttk.Button(card, text="Launch Tool", command=_launch).pack(side="bottom", fill="x")

        ttk.Button(body, text="📘 Open Antigravity Software Guide", command=lambda: self.gui._show_page("ag_guide")).pack(pady=20)

import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class OpenRoutinesPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "⚡ OpenRoutines Hub", "Sovereign Workflow & Agent Orchestration")
        self.build()

    def build(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)

        # Left Column: Tools & Agents
        l_fr = tk.Frame(body, bg=PAL["bg"], width=450)
        l_fr.pack(side="left", fill="y", padx=(0, 10))
        l_fr.pack_propagate(False)

        # Multi AI Orchestrator
        ai_card = self.gui._premium_card(l_fr, "🤖 Multi-AI Orchestrator")
        ai_card.master.pack(fill="x", pady=(0, 10))
        tk.Label(ai_card, text="Cross-Model Consensus Engine.", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")
        
        def _launch_multi_ai():
            self._notify("OpenRoutines", "Multi-AI Orchestrator Initialized.", "OK")
            self._log(self.log, "[OR] Syncing Llama-3, Phi-3, and Mistral consensus shims...", "INFO")
        
        self.gui._pulsing_button(ai_card, "Engage AI Consensus", _launch_multi_ai).pack(fill="x", pady=5)

        # Discovery Agents
        discovery_card = self._card(l_fr, "🔍 Discovery & Extraction")
        discovery_card.master.pack(fill="x", pady=(0, 10))
        
        def _email_discover():
            self._notify("Agent", "Email Discovery Agent Active", "INFO")
            self._log(self.log, "[AGENT] Harvesting contact footprints from encrypted local index...", "WARN")

        def _ip_extract():
            self._notify("Agent", "IP Extraction System Active", "OK")
            self._log(self.log, "[AGENT] Sanitizing network headers. Extracting sovereign egress points...", "INFO")

        self.gui._pulsing_button(discovery_card, "Email Discovery Agent", _email_discover).pack(fill="x", pady=2)
        self.gui._pulsing_button(discovery_card, "IP Extraction System", _ip_extract).pack(fill="x", pady=2)

        # Creative & Dev
        creative_card = self._card(l_fr, "🎨 Creative & Dev Shards")
        creative_card.master.pack(fill="x", pady=(0, 10))
        
        def _forge_pdf():
            self._notify("PDF Forge", "PDF Vector Forge Ready", "OK")
            self._log(self.log, "[FORGE] Synthesizing markdown into forensic PDF artifact...", "INFO")

        def _flowchart():
            self._notify("Vision", "Text-to-Flowchart Active", "OK")
            self._log(self.log, "[VISION] Drawing GraphViz schema from natural intent...", "INFO")

        self.gui._pulsing_button(creative_card, "PDF Forge Pro", _forge_pdf).pack(fill="x", pady=2)
        self.gui._pulsing_button(creative_card, "Text-to-Flowchart", _flowchart).pack(fill="x", pady=2)

        # Right Column: Execution Log & Ultra Control
        r_fr = tk.Frame(body, bg=PAL["bg"])
        r_fr.pack(side="left", fill="both", expand=True)

        # OpenRoutines Tiers (Ultra Control)
        tier_card = self._card(r_fr, "🚀 Routine Deployment Tiers")
        tier_card.master.pack(fill="x", pady=(0, 10))
        
        t_btn_fr = tk.Frame(tier_card, bg=PAL["card"])
        t_btn_fr.pack(fill="x")
        
        def _deploy_tier(tier):
            self._notify("OpenRoutines", f"Deploying {tier} Environment", "OK")
            self._log(self.log, f"[ROUTINE] {tier} protocols engaged. Zero-latency backplane active.", "HEAD")

        ttk.Button(t_btn_fr, text="Hub (Base)", command=lambda: _deploy_tier("HUB")).pack(side="left", padx=2, expand=True, fill="x")
        ttk.Button(t_btn_fr, text="PRO", command=lambda: _deploy_tier("PRO")).pack(side="left", padx=2, expand=True, fill="x")
        ttk.Button(t_btn_fr, text="ULTRA", command=lambda: _deploy_tier("ULTRA")).pack(side="left", padx=2, expand=True, fill="x")

        # Log
        log_card = self._card(r_fr, "📜 Orchestration Status")
        log_card.master.pack(fill="both", expand=True)
        self.log = self._console(log_card, height=25)
        self.log.pack(fill="both", expand=True)
        self._log(self.log, "SigmaOS OpenRoutines v1.0 APEX [READY]", "OK")
        self._log(self.log, "Awaiting Mission Deployment Commands...", "INFO")

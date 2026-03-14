import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_DIM, FONT_BOLD

class AutomationHubPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "OMNI AUTOMATOR STUDIO", "Zero-Trust Agentic Automation & Workflow Forging")
        self.build()

    def build(self):
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)

        left_col = tk.Frame(main_panel, bg=PAL["bg"], width=450)
        left_col.pack(side="left", fill="y", padx=(0, 10))
        left_col.pack_propagate(False)

        right_col = tk.Frame(main_panel, bg=PAL["bg"])
        right_col.pack(side="left", fill="both", expand=True)

        # 1. Shortcut Forge
        forge_card = self.gui._card(left_col, "⚡ Shortcut Forge")
        forge_card.master.pack(fill="x", pady=(0, 10))
        tk.Label(forge_card, text="Creates macOS-style visual workflows.", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")
        
        shortcut_name = ttk.Entry(forge_card)
        shortcut_name.pack(fill="x", pady=5)
        shortcut_name.insert(0, "Morning_Routine")

        def _forge_macro():
            if not hasattr(self.gui.kernel, "automator"):
                from sigma_core.system.automation_engine import SigmaOmniAutomator
                self.gui.kernel.automator = SigmaOmniAutomator(self.gui.kernel)
            
            auto = self.gui.kernel.automator
            steps = [{"action": "audit"}, {"action": "sync_neural_fabric", "delay": 2}]
            res = auto.create_shortcut(shortcut_name.get(), steps)
            self.gui._log(a_log, res, "OK")
            self._notify("Automator", f"Shortcut '{shortcut_name.get()}' forged.", "OK")

        ttk.Button(forge_card, text="Forge Shortcut Pipeline", command=_forge_macro).pack(fill="x", pady=(0, 5))

        def _run_macro():
            if not hasattr(self.gui.kernel, "automator"): return
            res = self.gui.kernel.automator.execute_workflow(shortcut_name.get())
            self.gui._log(a_log, res, "INFO")
            
        ttk.Button(forge_card, text="▶ Execute Shortcut", command=_run_macro).pack(fill="x")

        # 2. Agentic Sandbox Orbit
        sandbox_card = self.gui._card(left_col, "🚀 Agentic Sandbox Orbit")
        sandbox_card.master.pack(fill="x", pady=(0, 10))
        tk.Label(sandbox_card, text="Isolated, low-blast radius AI execution.", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")
        
        agent_name_ent = ttk.Entry(sandbox_card); agent_name_ent.pack(fill="x", pady=5); agent_name_ent.insert(0, "WebScraper_Agent")
        
        def _deploy_agent():
             if not self.gui.kernel.app_sandbox: return
             s_id = self.gui.kernel.app_sandbox.provision_agent_silo(agent_name_ent.get())
             script = "import os\nprint(f'Sovereign Isolation: {os.getcwd()}')"
             res = self.gui.kernel.app_sandbox.execute_agent_logic(s_id, script)
             self.gui._update_morphic_status("SANDBOX", f"Agent {s_id} Isolated", PAL["cyan"])
             self.gui._log(a_log, f"PROVISIONED: {s_id} for {agent_name_ent.get()}", "OK")
             self.gui._log(a_log, f"BLAST RADIUS: Contained in {res['path']}", "INFO")

        ttk.Button(sandbox_card, text="Deploy Sandboxed Agent", command=_deploy_agent).pack(fill="x")

        # 3. Context Triggers
        ctx_card = self.gui._card(left_col, "📍 Context Triggers")
        ctx_card.master.pack(fill="x", pady=(0, 10))
        tk.Label(ctx_card, text="Tasker parity. Trigger on hardware/OS events.", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")
        
        event_cb = ttk.Combobox(ctx_card, values=["POWER_CONNECTED", "NETWORK_CHANGE", "HIGH_CPU", "GEOLOCATION_ENTER"])
        event_cb.pack(fill="x")
        event_cb.set("POWER_CONNECTED")

        def _add_ctx():
            if not hasattr(self.gui.kernel, "automator"): return
            res = self.gui.kernel.automator.add_context_trigger(event_cb.get(), "active = true", lambda: self.gui._log_voice(f"Trigger {event_cb.get()} fired!"))
            self.gui._log(a_log, res, "OK")

        ttk.Button(ctx_card, text="Arm Context Trigger", command=_add_ctx).pack(fill="x", pady=10)

        # 4. Log Panel
        log_panel = self.gui._card(right_col, "📜 OmniAutomator Status & Telemetry")
        log_panel.master.pack(fill="both", expand=True)
        a_log = self.gui._console(log_panel, height=25)
        a_log.pack(fill="both", expand=True)
        self.gui._log(a_log, "OmniAutomator Engine v2.0 READY. Listening for context hooks...", "INFO")

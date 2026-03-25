import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL

class AetherOrchPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "AETHER ORCHESTRATOR", "Unified AI Coordination & Cross-Model Intent Routing")

        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True)

        # 1. AI Command Center (Left)
        l_fr = tk.Frame(body, bg=PAL["bg2"], width=400)
        l_fr.pack(side="left", fill="both", padx=10, pady=10)
        l_fr.pack_propagate(False)

        aether = self.controller.kernel.registry.get("aether_orch")
        
        c_card = self.controller._card(l_fr, "AI Orchestration Panel")
        c_card.master.pack(fill="x", pady=10)
        
        tk.Label(c_card, text="Direct Aether Prompt:", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")
        a_ent = ttk.Entry(c_card)
        a_ent.pack(fill="x", pady=5)
        a_ent.insert(0, "Analyze recent security emails and suggest routines.")
        
        def _exec_aether():
            if not aether: return
            res = aether.collaborative_inference(a_ent.get())
            self.controller._log(a_log, f"\n[AETHER] Collaborative Response:", "HEAD")
            self.controller._log(a_log, res["collaborative_summary"], "OK")
            self.controller._log(a_log, f"  → Suggested Routine: {res['proposed_routine']}", "INFO")
            btn_rout.config(text=f"LAUNCH: {res['proposed_routine']}", command=lambda: self.controller._run_routine(res["proposed_routine"].lower()))

        ttk.Button(c_card, text="EXECUTE COLLABORATIVE AI", style="Teal.TButton", command=_exec_aether).pack(fill="x", pady=5)
        
        btn_rout = ttk.Button(c_card, text="PROPOSED ROUTINE (N/A)", state="disabled")
        btn_rout.pack(fill="x", pady=5)

        # Integration status
        i_card = self.controller._card(l_fr, "Integrated Agents")
        i_card.master.pack(fill="x", pady=10)
        
        for agent in [("Email Discovery Agent", "sigma.ai.email_disco"), 
                      ("Excel AI Filler", "sigma.prod.excel_ai"), 
                      ("Aether Mesh Node", "sigma.mesh")]:
            fr = tk.Frame(i_card, bg=PAL["card"])
            fr.pack(fill="x", pady=2)
            tk.Label(fr, text=agent[0], font=FONT_SMALL, fg=PAL["text"], bg=PAL["card"]).pack(side="left")
            tk.Label(fr, text=" [ACTIVE]", font=FONT_SMALL, fg=PAL["green"], bg=PAL["card"]).pack(side="right")

        # 2. Aether Output & Log (Right)
        r_fr = tk.Frame(body, bg=PAL["bg"])
        r_fr.pack(side="left", fill="both", expand=True, padx=10, pady=10)

        a_log = self.controller._console(r_fr, height=30)
        a_log.pack(fill="both", expand=True)
        self.controller._log(a_log, "Aether core online. Quantum intent routing ready for collaborative analysis.", "INFO")

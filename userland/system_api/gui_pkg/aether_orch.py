import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL

class AetherOrchPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "AETHER ORCHESTRATOR", "Unified AI Coordination & Cross-Model Intent Routing")
        self.build()

    def build(self):

        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True)

        # 1. AI Command Center (Left)
        l_fr = tk.Frame(body, bg=PAL["bg2"], width=400)
        l_fr.pack(side="left", fill="both", padx=10, pady=10)
        l_fr.pack_propagate(False)

        aether = self.gui.kernel.registry.get("aether_orch")
        
        c_card = self.gui._card(l_fr, "AI Orchestration Panel")
        c_card.master.pack(fill="x", pady=10)
        
        tk.Label(c_card, text="Direct Aether Prompt:", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")
        a_ent = ttk.Entry(c_card)
        a_ent.pack(fill="x", pady=5)
        a_ent.insert(0, "Analyze recent security emails and suggest routines.")
        
        def _exec_aether():
            if not aether: return
            res = aether.collaborative_inference(a_ent.get())
            self.gui._log(a_log, f"\n[AETHER] Collaborative Response:", "HEAD")
            self.gui._log(a_log, res["collaborative_summary"], "OK")
            self.gui._log(a_log, f"  → Suggested Routine: {res['proposed_routine']}", "INFO")
            btn_rout.config(text=f"LAUNCH: {res['proposed_routine']}", command=lambda: self.gui._run_routine(res["proposed_routine"].lower()))

        ttk.Button(c_card, text="EXECUTE COLLABORATIVE AI", style="Teal.TButton", command=_exec_aether).pack(fill="x", pady=5)
        
        btn_rout = ttk.Button(c_card, text="PROPOSED ROUTINE (N/A)", state="disabled")
        btn_rout.pack(fill="x", pady=5)

        # Integration status
        i_card = self.gui._card(l_fr, "Integrated Agents")
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

        # COMPETITOR CRUSHER (USP)
        crush_c = self.gui._card(r_fr, "🛸 Sovereign Competitor Crusher (AI USP)")
        crush_c.master.pack(fill="x", pady=(0, 10))
        
        tk.Label(crush_c, text="Target OS Models: Windows 11 Copilot, macOS Sequoia", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")
        ttk.Button(crush_c, text="Launch Benchmark War-Room", 
                   command=lambda: self.gui._show_page("war_room")).pack(side="left", padx=5, pady=5)
        
        # NEURAL CONTENT SANITIZER (USP)
        san_c = self.gui._card(r_fr, "🧠 Neural Content Sanitizer (ML USP)")
        san_c.master.pack(fill="x", pady=10)
        tk.Label(san_c, text="Sanitization Mode: ADAPTIVE (Child-Safe + Professional)", font=FONT_SMALL, fg=PAL["teal"], bg=PAL["card"]).pack(anchor="w")
        
        def toggle_san():
            self.gui.kernel.registry.get("aether_orch").toggle_sanitization()
            self.gui._log(a_log, "[AI] Neural Sanitizer threshold adjusted for peak cognitive flow.", "OK")
            
        ttk.Button(san_c, text="Re-Calibrate Sanitizer", command=toggle_san).pack(side="left", padx=5, pady=5)

        a_log = self.gui._console(r_fr, height=20)
        a_log.pack(fill="both", expand=True)
        self.gui._log(a_log, "Aether core online. Quantum intent routing ready for collaborative analysis.", "INFO")

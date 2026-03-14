import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class FlowPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "🌿 SigmaFlowAI", "Procedural Logic Architect & Auditor")
        self.build()

    def build(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)

        # 1. Inputs (Left)
        l_fr = tk.Frame(body, bg=PAL["bg2"], width=350)
        l_fr.pack(side="left", fill="both", padx=5)
        l_fr.pack_propagate(False)

        tk.Label(l_fr, text="Define Procedure", font=FONT_MED, fg=PAL["gold"], bg=PAL["bg2"]).pack(pady=10)
        self.raw_txt = tk.Text(l_fr, font=FONT_SMALL, bg=PAL["bg"], fg=PAL["text"], height=15)
        self.raw_txt.pack(fill="both", expand=True, padx=10, pady=5)
        self.raw_txt.insert("1.0", "Start -> Process -> End")

        # 2. Output (Right)
        r_fr = tk.Frame(body, bg=PAL["bg"])
        r_fr.pack(side="left", fill="both", expand=True, padx=5)

        m_card = self._card(r_fr, "🗺️ Process Map (Mermaid)")
        m_card.master.pack(fill="both", expand=True)
        self.log = self._console(m_card, height=20)
        self.log.pack(fill="both", expand=True)

        def gen_flow():
            self._log(self.log, "Generating logical flow...", "HEAD")
            self.after(500, lambda: self._log(self.log, "graph TD\nA -> B\nB -> C", "OK"))

        ttk.Button(l_fr, text="🧬 Generate Logic Flow", command=gen_flow).pack(pady=10)

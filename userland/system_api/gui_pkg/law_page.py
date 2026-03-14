import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class LawPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "SigmaLawyer Pro", "The Universal Legal Operating System")
        self.build()

    def build(self):
        tab_bar = tk.Frame(self, bg=PAL["bg2"], height=40)
        tab_bar.pack(fill="x", pady=(0,10))
        tab_bar.pack_propagate(False)
        
        tabs = [
            ("Research",   "🔍"),
            ("JurisPro",   "📜"),
            ("Litigation", "⚖️"),
            ("Outcome",    "🔮"),
            ("PublicLaw",  "📢"),
            ("Drafting",   "📑"),
            ("Calculators","🧮")
        ]
        
        self.container = tk.Frame(self, bg=PAL["bg"])
        self.container.pack(fill="both", expand=True)
        self.sub_pages = {}
        
        for name, icon in tabs:
            b = tk.Button(tab_bar, text=f"{icon} {name}", font=FONT_SMALL, fg=PAL["text"],
                          bg=PAL["bg2"], bd=0, activebackground=PAL["bg"], 
                          command=lambda n=name.lower(): self._show_sub(n))
            b.pack(side="left", padx=10, fill="y")

        self._show_sub("research")

    def _show_sub(self, name):
        for s in self.sub_pages.values(): s.pack_forget()
        if name not in self.sub_pages:
            p = tk.Frame(self.container, bg=PAL["bg"])
            self.sub_pages[name] = p
            getattr(self, f"_build_{name}")(p)
        self.sub_pages[name].pack(fill="both", expand=True)

    def _build_research(self, parent):
        l_fr = tk.Frame(parent, bg=PAL["bg2"], width=350)
        l_fr.pack(side="left", fill="both", padx=5, pady=5)
        l_fr.pack_propagate(False)
        
        tk.Label(l_fr, text="Bare Act / CaseIQ", font=FONT_MED, fg=PAL["gold"], bg=PAL["bg2"]).pack(pady=5)
        s_ent = ttk.Entry(l_fr); s_ent.pack(fill="x", padx=10); s_ent.insert(0, "BNSS_2023 Section 154")
        
        res_text = tk.Text(l_fr, font=FONT_SMALL, bg=PAL["bg"], fg=PAL["text"], height=15)
        res_text.pack(fill="both", expand=True, padx=10, pady=5)
        
        def do_research():
            txt = s_ent.get()
            q =  self.kernel.law.ai_case_iq(txt)
            res_text.delete("1.0", "end")
            res_text.insert("end", f"CaseIQ Suggestions:\n" + "─"*20 + "\n")
            for r in q: res_text.insert("end", f"• {r.get('Reference')}: {r.get('Meaning')}\n\n")

        ttk.Button(l_fr, text="Execute CaseIQ Search", command=do_research).pack(pady=5)
        
        r_fr = tk.Frame(parent, bg=PAL["bg"])
        r_fr.pack(side="left", fill="both", expand=True, padx=5, pady=5)
        tk.Label(r_fr, text="Internal Case Database", font=FONT_MED, fg=PAL["cyan"], bg=PAL["bg"]).pack(anchor="w")
        
        web_f = tk.Frame(r_fr, bg=PAL["bg"])
        web_f.pack(fill="x", pady=10)
        for site in ["IndianKanoon", "IndiaCode"]:
            ttk.Button(web_f, text=f"Sync {site}", command=lambda: self.gui._notify("Law", f"Syncing with {site}...", "OK")).pack(side="left", padx=5)

    def _build_jurispro(self, parent):
        tk.Label(parent, text="Jurisprudential Analysis Hub", font=FONT_MED, fg=PAL["gold"], bg=PAL["bg"]).pack(pady=10)
        txt = tk.Text(parent, bg=PAL["bg2"], fg=PAL["gold"], font=FONT_MED, height=15)
        txt.pack(fill="both", expand=True, padx=20, pady=10)
        txt.insert("1.0", "Select a legal school for analysis...")

    def _build_litigation(self, parent):
        tk.Label(parent, text="Litigation & E-Discovery", font=FONT_MED, fg=PAL["cyan"], bg=PAL["bg"]).pack(pady=10)
        log = self.gui._console(parent, height=15)
        log.pack(fill="both", expand=True, padx=20, pady=10)
        ttk.Button(parent, text="Launch Discovery Scan", command=lambda: self.gui._log(log, "Searching encrypted silos...", "INFO")).pack(pady=10)

    def _build_outcome(self, parent):
        tk.Label(parent, text="Predictive Outcome Simulation", font=FONT_MED, fg=PAL["accent"], bg=PAL["bg"]).pack(pady=10)
        view = tk.Text(parent, bg=PAL["bg2"], fg=PAL["text"], height=12)
        view.pack(fill="x", padx=20, pady=10)
        ttk.Button(parent, text="Simulate Outcome", command=lambda: self.gui._notify("APEX", "Running Monte Carlo simulation...", "INFO")).pack()

    def _build_publiclaw(self, parent):
        tk.Label(parent, text="Plain Language Law (Nyaaya)", font=FONT_MED, fg=PAL["teal"], bg=PAL["bg"]).pack(pady=10)
        tk.Label(parent, text="Explain 'FIR' / 'Bail' / 'Contract' and more.", fg=PAL["dim"], bg=PAL["bg"]).pack()

    def _build_drafting(self, parent):
        tk.Label(parent, text="Legal Drafting Workbench", font=FONT_MED, fg=PAL["gold"], bg=PAL["bg"]).pack(pady=10)
        box = tk.Text(parent, font=("Courier New", 10), bg=PAL["bg2"], fg=PAL["text"], height=15)
        box.pack(fill="both", expand=True, padx=20, pady=10)

    def _build_calculators(self, parent):
        tk.Label(parent, text="Statutory Financial Calculators", font=FONT_MED, fg=PAL["gold"], bg=PAL["bg"]).pack(pady=10)
        ttk.Button(parent, text="Calculate FY25 Tax").pack(pady=5)
        ttk.Button(parent, text="Calculate Gratuity").pack(pady=5)

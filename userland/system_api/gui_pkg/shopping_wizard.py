import tkinter as tk
from tkinter import ttk
import random
import webbrowser
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_TITLE

class ShoppingWizardPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "🛒 BuyingHatke Wizard", "Sovereign Price Intelligence & Enterprise Commerce Hub")
        self._build_ui()

    def _build_ui(self):
        # Master Container
        container = tk.Frame(self, bg=PAL["bg"])
        container.pack(fill="both", expand=True, padx=20, pady=20)

        # ─── Navigation Header ───
        header = tk.Frame(container, bg=PAL["card"], height=60)
        header.pack(fill="x", pady=(0, 20))
        header.pack_propagate(False)

        tk.Label(header, text="Product Analyzer:", font=FONT_MED, fg=PAL["cyan"], bg=PAL["card"]).pack(side="left", padx=20)
        self.prod_var = tk.StringVar(value="iPhone 15 Pro")
        ent = ttk.Entry(header, textvariable=self.prod_var, width=40)
        ent.pack(side="left", padx=10, ipady=5)

        ttk.Button(header, text="🔍 Analyze Market", command=self._analyze).pack(side="left", padx=10)
        ttk.Button(header, text="🎟️ Auto-Discover Coupons", command=self._coupons).pack(side="left", padx=10)

        # ─── Main Content Grid ───
        main = tk.Frame(container, bg=PAL["bg"])
        main.pack(fill="both", expand=True)

        # Left: Live Intel & Forecast
        left = tk.Frame(main, bg=PAL["bg"])
        left.pack(side="left", fill="both", expand=True, padx=(0, 10))

        # Price Intel
        intel_fr = self._card(left, "Live Product Intel (BuyHatke Engine)")
        intel_fr.master.pack(fill="x", pady=(0, 10))
        self.intel_txt = tk.Text(intel_fr, height=10, bg=PAL["bg"], fg=PAL["text"], font=FONT_SMALL, bd=0)
        self.intel_txt.pack(fill="x", padx=10, pady=10)

        # Forecast
        fore_fr = self._card(left, "Quantum Price Forecasting")
        fore_fr.master.pack(fill="x")
        self.fore_txt = tk.Text(fore_fr, height=8, bg=PAL["bg2"], fg=PAL["gold"], font=FONT_SMALL, bd=0)
        self.fore_txt.pack(fill="x", padx=10, pady=10)

        # Right: Commercial Strategy & B2B
        right = tk.Frame(main, bg=PAL["bg"], width=350)
        right.pack(side="right", fill="both")
        right.pack_propagate(False)

        # USP Analysis
        usp_fr = self._card(right, "AI Strategy Matrix (Praxie)")
        usp_fr.master.pack(fill="x", pady=(0, 10))
        self.usp_txt = tk.Text(usp_fr, height=12, bg=PAL["card"], fg=PAL["cyan"], font=FONT_SMALL, bd=0)
        self.usp_txt.pack(fill="x", padx=5, pady=5)

        # B2B & CRM
        crm_fr = self._card(right, "Sovereign CRM Pipeline")
        crm_fr.master.pack(fill="x")
        self.crm_txt = tk.Text(crm_fr, height=10, bg=PAL["bg3"], fg=PAL["text"], font=FONT_SMALL, bd=0)
        self.crm_txt.pack(fill="x", padx=5, pady=5)

        self.gui.after(500, self._analyze) # Initial run deferred for hydration

    def _analyze(self):
        prod = self.prod_var.get()
        engine = self.kernel.registry.get("buyhatke")
        if not engine:
            self._notify("Error", "BuyHatke Engine not found in registry.", "ERR")
            return

        res = engine.analyze_deal(prod, 69900)
        self.intel_txt.delete("1.0", tk.END)
        self.intel_txt.insert(tk.END, f"ANALYSIS FOR: {res['Product']}\n" + "─"*30 + "\n")
        self.intel_txt.insert(tk.END, f"Verdict: {res['Verdict']}\n")
        self.intel_txt.insert(tk.END, f"Lowest Ever: ₹{res['Lowest_Ever']}\n")
        self.intel_txt.insert(tk.END, f"Average: ₹{res['Average']}\n")
        self.intel_txt.insert(tk.END, f"Savings Potential: ₹{res['Savings_Potential']}\n")

        # Forecast
        f_res = engine.quantum_price_forecast(prod)
        self.fore_txt.delete("1.0", tk.END)
        self.fore_txt.insert(tk.END, "PREDICTIVE TRENDS:\n" + "─"*30 + "\n")
        for k, v in f_res.items():
            self.fore_txt.insert(tk.END, f"{k}: {v}\n")

        # Strategy
        s_res = engine.analyze_usp_matrix("Retail")
        self.usp_txt.delete("1.0", tk.END)
        for k, v in s_res.items():
            self.usp_txt.insert(tk.END, f"▶ {k}:\n  {v}\n\n")

        # CRM
        leads = engine.crm_lead_pipeline()
        self.crm_txt.delete("1.0", tk.END)
        for l in leads:
            self.crm_txt.insert(tk.END, f"👤 {l['Lead']} | Score: {l['Score']} | {l['Status']}\n")

    def _coupons(self):
        engine = self.kernel.registry.get("buyhatke")
        if engine:
            cs = engine.find_coupons("Global")
            self._notify("Coupon Discovery", f"Verified Coupons Found: {', '.join(cs)}", "OK")

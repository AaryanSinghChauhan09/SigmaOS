# Generated method: ShoppingWizardPage._analyze
import tkinter as tk
from tkinter import ttk
import random
import webbrowser
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_TITLE

class ShoppingWizardPage:
    def _analyze(self):
        prod = self.prod_var.get()
        engine = self.kernel.registry.get('buyhatke')
        if not engine:
            self._notify('Error', 'BuyHatke Engine not found in registry.', 'ERR')
            return
        res = engine.analyze_deal(prod, 69900)
        self.intel_txt.delete('1.0', tk.END)
        self.intel_txt.insert(tk.END, f"ANALYSIS FOR: {res['Product']}\n" + '─' * 30 + '\n')
        self.intel_txt.insert(tk.END, f"Verdict: {res['Verdict']}\n")
        self.intel_txt.insert(tk.END, f"Lowest Ever: ₹{res['Lowest_Ever']}\n")
        self.intel_txt.insert(tk.END, f"Average: ₹{res['Average']}\n")
        self.intel_txt.insert(tk.END, f"Savings Potential: ₹{res['Savings_Potential']}\n")
        f_res = engine.quantum_price_forecast(prod)
        self.fore_txt.delete('1.0', tk.END)
        self.fore_txt.insert(tk.END, 'PREDICTIVE TRENDS:\n' + '─' * 30 + '\n')
        for k, v in f_res.items():
            self.fore_txt.insert(tk.END, f'{k}: {v}\n')
        s_res = engine.analyze_usp_matrix('Retail')
        self.usp_txt.delete('1.0', tk.END)
        for k, v in s_res.items():
            self.usp_txt.insert(tk.END, f'▶ {k}:\n  {v}\n\n')
        leads = engine.crm_lead_pipeline()
        self.crm_txt.delete('1.0', tk.END)
        for l in leads:
            self.crm_txt.insert(tk.END, f"👤 {l['Lead']} | Score: {l['Score']} | {l['Status']}\n")
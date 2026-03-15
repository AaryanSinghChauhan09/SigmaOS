# Generated method: ShoppingWizardPage._build_ui
import tkinter as tk
from tkinter import ttk
import random
import webbrowser
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_TITLE

class ShoppingWizardPage:
    def _build_ui(self):
        container = tk.Frame(self, bg=PAL['bg'])
        container.pack(fill='both', expand=True, padx=20, pady=20)
        header = tk.Frame(container, bg=PAL['card'], height=60)
        header.pack(fill='x', pady=(0, 20))
        header.pack_propagate(False)
        tk.Label(header, text='Product Analyzer:', font=FONT_MED, fg=PAL['cyan'], bg=PAL['card']).pack(side='left', padx=20)
        self.prod_var = tk.StringVar(value='iPhone 15 Pro')
        ent = ttk.Entry(header, textvariable=self.prod_var, width=40)
        ent.pack(side='left', padx=10, ipady=5)
        ttk.Button(header, text='🔍 Analyze Market', command=self._analyze).pack(side='left', padx=10)
        ttk.Button(header, text='🎟️ Auto-Discover Coupons', command=self._coupons).pack(side='left', padx=10)
        main = tk.Frame(container, bg=PAL['bg'])
        main.pack(fill='both', expand=True)
        left = tk.Frame(main, bg=PAL['bg'])
        left.pack(side='left', fill='both', expand=True, padx=(0, 10))
        intel_fr = self._card(left, 'Live Product Intel (BuyHatke Engine)')
        intel_fr.master.pack(fill='x', pady=(0, 10))
        self.intel_txt = tk.Text(intel_fr, height=10, bg=PAL['bg'], fg=PAL['text'], font=FONT_SMALL, bd=0)
        self.intel_txt.pack(fill='x', padx=10, pady=10)
        fore_fr = self._card(left, 'Quantum Price Forecasting')
        fore_fr.master.pack(fill='x')
        self.fore_txt = tk.Text(fore_fr, height=8, bg=PAL['bg2'], fg=PAL['gold'], font=FONT_SMALL, bd=0)
        self.fore_txt.pack(fill='x', padx=10, pady=10)
        right = tk.Frame(main, bg=PAL['bg'], width=350)
        right.pack(side='right', fill='both')
        right.pack_propagate(False)
        usp_fr = self._card(right, 'AI Strategy Matrix (Praxie)')
        usp_fr.master.pack(fill='x', pady=(0, 10))
        self.usp_txt = tk.Text(usp_fr, height=12, bg=PAL['card'], fg=PAL['cyan'], font=FONT_SMALL, bd=0)
        self.usp_txt.pack(fill='x', padx=5, pady=5)
        crm_fr = self._card(right, 'Sovereign CRM Pipeline')
        crm_fr.master.pack(fill='x')
        self.crm_txt = tk.Text(crm_fr, height=10, bg=PAL['bg3'], fg=PAL['text'], font=FONT_SMALL, bd=0)
        self.crm_txt.pack(fill='x', padx=5, pady=5)
        self.gui.after(500, self._analyze)
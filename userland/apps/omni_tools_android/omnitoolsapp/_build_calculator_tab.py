"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._build_calculator_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _build_calculator_tab(self) -> None:
        tk.Label(self.tab_calc, text='Multi-Purpose Offline Calculators', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        ef = tk.Frame(self.tab_calc, bg=PAL['panel'], padx=15, pady=15)
        ef.pack(fill='x', pady=5)
        tk.Label(ef, text='Expression  (e.g. 2*3+sqrt(16)):', bg=PAL['panel'], fg=PAL['text']).grid(row=0, column=0, sticky='e')
        self.expr_entry = tk.Entry(ef, width=35, font=('Consolas', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.expr_entry.grid(row=0, column=1, padx=8)
        tk.Button(ef, text='EVAL', bg=PAL['success'], fg='black', command=self._eval_expr).grid(row=0, column=2)
        self.expr_result = tk.Label(ef, text='Result: —', bg=PAL['panel'], fg=PAL['dim'], font=('Inter', 10, 'bold'))
        self.expr_result.grid(row=1, column=0, columnspan=3, pady=8)
        loan_f = tk.LabelFrame(self.tab_calc, text='Loan & EMI Calculator', bg=PAL['panel'], fg=PAL['text'], font=('Inter', 10, 'bold'))
        loan_f.pack(fill='x', pady=10, padx=5)
        fields = [('Principal $', 'loan_principal_entry'), ('Annual Rate %', 'loan_rate_entry'), ('Years', 'loan_years_entry')]
        for i, (lbl, attr) in enumerate(fields):
            tk.Label(loan_f, text=lbl, bg=PAL['panel'], fg=PAL['dim']).grid(row=i, column=0, sticky='e', pady=4, padx=5)
            e = tk.Entry(loan_f, width=14, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
            e.grid(row=i, column=1, pady=4)
            setattr(self, attr, e)
        tk.Button(loan_f, text='CALCULATE EMI', bg=PAL['accent_dim'], fg='black', command=self._calc_emi).grid(row=3, column=0, columnspan=2, pady=8)
        self.emi_result = tk.Label(loan_f, text='EMI: —', bg=PAL['panel'], fg=PAL['dim'], font=('Inter', 10, 'bold'))
        self.emi_result.grid(row=4, column=0, columnspan=2, pady=4)

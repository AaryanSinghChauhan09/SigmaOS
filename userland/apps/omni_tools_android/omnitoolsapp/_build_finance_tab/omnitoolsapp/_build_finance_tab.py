# Generated method: OmniToolsApp._build_finance_tab
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _build_finance_tab(self) -> None:
        tk.Label(self.tab_fin, text='Financial & Personal Utilities', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        sf = tk.LabelFrame(self.tab_fin, text='Smart Bill Splitter', bg=PAL['panel'], fg=PAL['text'])
        sf.pack(fill='x', pady=5, padx=5)
        tk.Label(sf, text='Total amount $:', bg=PAL['panel'], fg=PAL['dim']).grid(row=0, column=0, sticky='e', pady=4, padx=5)
        self.split_total = tk.Entry(sf, width=12, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.split_total.grid(row=0, column=1, pady=4)
        tk.Label(sf, text='People:', bg=PAL['panel'], fg=PAL['dim']).grid(row=1, column=0, sticky='e', pady=4, padx=5)
        self.split_people = tk.Entry(sf, width=6, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.split_people.grid(row=1, column=1, pady=4)
        tk.Button(sf, text='CALCULATE', bg=PAL['success'], fg='black', command=self._calc_split).grid(row=2, column=0, columnspan=2, pady=8)
        self.split_result = tk.Label(sf, text='Each pays: —', bg=PAL['panel'], fg=PAL['dim'], font=('Inter', 10, 'bold'))
        self.split_result.grid(row=3, column=0, columnspan=2, pady=4)
        bf = tk.LabelFrame(self.tab_fin, text='BMI Calculator', bg=PAL['panel'], fg=PAL['text'])
        bf.pack(fill='x', pady=5, padx=5)
        tk.Label(bf, text='Weight (kg):', bg=PAL['panel'], fg=PAL['dim']).grid(row=0, column=0, sticky='e', pady=4, padx=5)
        self.bmi_weight = tk.Entry(bf, width=8, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.bmi_weight.grid(row=0, column=1, pady=4)
        tk.Label(bf, text='Height (cm):', bg=PAL['panel'], fg=PAL['dim']).grid(row=1, column=0, sticky='e', pady=4, padx=5)
        self.bmi_height = tk.Entry(bf, width=8, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.bmi_height.grid(row=1, column=1, pady=4)
        tk.Button(bf, text='CALCULATE', bg=PAL['accent_dim'], fg='black', command=self._calc_bmi).grid(row=2, column=0, columnspan=2, pady=8)
        self.bmi_result = tk.Label(bf, text='BMI: —', bg=PAL['panel'], fg=PAL['dim'], font=('Inter', 10, 'bold'))
        self.bmi_result.grid(row=3, column=0, columnspan=2, pady=4)
        pf = tk.LabelFrame(self.tab_fin, text='Secure Password Generator  (stdlib secrets)', bg=PAL['panel'], fg=PAL['text'])
        pf.pack(fill='x', pady=5, padx=5)
        tk.Label(pf, text='Length:', bg=PAL['panel'], fg=PAL['dim']).grid(row=0, column=0, sticky='e', pady=4, padx=5)
        self.pwd_len = tk.Spinbox(pf, from_=8, to=64, width=6, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], buttonbackground=PAL['accent_dim'], relief='flat')
        self.pwd_len.grid(row=0, column=1, pady=4)
        tk.Button(pf, text='GENERATE & COPY', bg=PAL['success'], fg='black', command=self._gen_password).grid(row=1, column=0, columnspan=2, pady=8)
        self.pwd_result = tk.Entry(pf, width=45, font=('Consolas', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.pwd_result.grid(row=2, column=0, columnspan=2, pady=4)
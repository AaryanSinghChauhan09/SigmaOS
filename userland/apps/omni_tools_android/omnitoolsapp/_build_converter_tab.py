"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._build_converter_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _build_converter_tab(self) -> None:
        tk.Label(self.tab_converter, text='Offline Unit & Currency Converters', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        lf = tk.LabelFrame(self.tab_converter, text='Length  (m ↔ ft)', bg=PAL['panel'], fg=PAL['text'])
        lf.pack(fill='x', pady=5, padx=5)
        tk.Label(lf, text='Metres:', bg=PAL['panel'], fg=PAL['dim']).grid(row=0, column=0, sticky='e', pady=4, padx=5)
        self.meter_entry = tk.Entry(lf, width=12, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.meter_entry.grid(row=0, column=1, padx=5)
        tk.Button(lf, text='→ Feet', bg=PAL['accent_dim'], fg='black', command=self._m_to_ft).grid(row=0, column=2, padx=5)
        tk.Label(lf, text='Feet:', bg=PAL['panel'], fg=PAL['dim']).grid(row=1, column=0, sticky='e', pady=4, padx=5)
        self.feet_entry = tk.Entry(lf, width=12, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.feet_entry.grid(row=1, column=1, padx=5)
        tk.Button(lf, text='→ Metres', bg=PAL['accent_dim'], fg='black', command=self._ft_to_m).grid(row=1, column=2, padx=5)
        cf = tk.LabelFrame(self.tab_converter, text='Currency  (USD ↔ EUR, offline rates)', bg=PAL['panel'], fg=PAL['text'])
        cf.pack(fill='x', pady=5, padx=5)
        tk.Label(cf, text='USD $:', bg=PAL['panel'], fg=PAL['dim']).grid(row=0, column=0, sticky='e', pady=4, padx=5)
        self.usd_entry = tk.Entry(cf, width=12, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.usd_entry.grid(row=0, column=1, padx=5)
        tk.Button(cf, text='→ EUR', bg=PAL['accent_dim'], fg='black', command=self._usd_to_eur).grid(row=0, column=2, padx=5)
        tk.Label(cf, text='EUR €:', bg=PAL['panel'], fg=PAL['dim']).grid(row=1, column=0, sticky='e', pady=4, padx=5)
        self.eur_entry = tk.Entry(cf, width=12, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.eur_entry.grid(row=1, column=1, padx=5)
        tk.Button(cf, text='→ USD', bg=PAL['accent_dim'], fg='black', command=self._eur_to_usd).grid(row=1, column=2, padx=5)
        tmpf = tk.LabelFrame(self.tab_converter, text='Temperature', bg=PAL['panel'], fg=PAL['text'])
        tmpf.pack(fill='x', pady=5, padx=5)
        tk.Label(tmpf, text='Value:', bg=PAL['panel'], fg=PAL['dim']).grid(row=0, column=0, sticky='e', padx=5)
        self.temp_in = tk.Entry(tmpf, width=10, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.temp_in.grid(row=0, column=1, padx=5)
        self.temp_unit = tk.StringVar(value='C→F')
        ttk.Combobox(tmpf, textvariable=self.temp_unit, width=8, values=['C→F', 'F→C', 'C→K', 'K→C']).grid(row=0, column=2, padx=5)
        tk.Button(tmpf, text='CONVERT', bg=PAL['success'], fg='black', command=self._convert_temp).grid(row=0, column=3, padx=5)
        self.temp_out = tk.Label(tmpf, text='Result: —', bg=PAL['panel'], fg=PAL['accent'], font=('Inter', 10, 'bold'))
        self.temp_out.grid(row=1, column=0, columnspan=4, pady=6)

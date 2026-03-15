"""
Auto-split from userland\apps\omni_savant.py — OmniSavant._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class OmniSavant:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='OMNI-SAVANT ARCHIVES', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='🔬 THEORETICAL SANDBOX', font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=self._mock_sandbox).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.domain_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=280, padx=15, pady=15)
        self.domain_fr.pack(side='left', fill='y', padx=(0, 20))
        self.domain_fr.pack_propagate(False)
        tk.Label(self.domain_fr, text='KNOWLEDGE DOMAINS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        for domain in self.knowledge_base.keys():
            lbl = tk.Label(self.domain_fr, text=f'💠 {domain}', font=('Inter', 10, 'bold'), fg=PAL['text'], bg=PAL['sidebar'], padx=10, pady=10, cursor='hand2')
            lbl.pack(fill='x', pady=5)
            lbl.bind('<Button-1>', lambda e, d=domain: self._load_domain(d))
        self.concept_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.concept_fr.pack(side='left', fill='both', expand=True)
        self.dom_title = tk.Label(self.concept_fr, text='SELECT A DOMAIN MATRIX', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg'])
        self.dom_title.pack(anchor='w', pady=(0, 10))
        cols = ('Concept Architecture', 'Core Theorems', 'SigmaOS Implementation')
        self.tree = ttk.Treeview(self.concept_fr, columns=cols, show='headings', style='Savant.Treeview')
        widths = [200, 250, 300]
        for c, w in zip(cols, widths):
            self.tree.heading(c, text=c.upper())
            self.tree.column(c, width=w, anchor='w')
        self.tree.pack(fill='both', expand=True)
        self.tree.bind('<Double-1>', self._inspect_concept)
        self.status = tk.Label(self, text='SAVANT ENGINE IDLE | CONNECTED TO NEURAL KNOWLEDGE GRAPH', bg=PAL['accent_dim'], fg='black', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
        self._load_domain('Computer Science (CS)')

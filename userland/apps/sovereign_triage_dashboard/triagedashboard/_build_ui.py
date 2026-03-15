"""
Auto-split from userland\apps\sovereign_triage_dashboard.py — TriageDashboard._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox
from typing import Dict, Any, List, Optional
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT



class TriageDashboard:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['background'], height=100, padx=40)
        self.header.pack(side='top', fill='x', pady=20)
        tk.Label(self.header, text='SOVEREIGN TRIAGE', font=FONT['h1'], fg=PAL['accent'], bg=PAL['background']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['background'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='⚖️ FILE COMPLAINT', bg=PAL['accent'], fg='white', font=FONT['caption'], relief='flat', padx=20, pady=10, command=self._mock_file_complaint).pack(side='left', padx=5)
        tk.Button(btn_fr, text='🔄 REFRESH DOCKET', bg=PAL['surface_variant'], fg=PAL['text_primary'], font=FONT['caption'], relief='flat', padx=20, pady=10, command=self._refresh_data).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['background'], padx=40)
        self.workspace.pack(fill='both', expand=True)
        self.stats_fr = tk.Frame(self.workspace, bg=PAL['background'], height=150)
        self.stats_fr.pack(side='top', fill='x', pady=(0, 25))
        self.stat_cards = []
        labels = ['CASES FILED', 'JUDGMENTS DELIVERED', 'PENDING TRIALS', 'MEAN CLOSURE TIME']
        for lbl in labels:
            f = tk.Frame(self.stats_fr, bg=PAL['surface'], padx=25, pady=25)
            f.pack(side='left', expand=True, fill='both', padx=8)
            tk.Label(f, text=lbl, font=FONT['caption'], fg=PAL['text_secondary'], bg=PAL['surface']).pack(anchor='w')
            val_lbl = tk.Label(f, text='0', font=FONT['h2'], fg=PAL['text_primary'], bg=PAL['surface'])
            val_lbl.pack(anchor='w')
            self.stat_cards.append(val_lbl)
        self.docket_fr = tk.Frame(self.workspace, bg=PAL['surface'], padx=20, pady=20)
        self.docket_fr.pack(side='top', fill='both', expand=True)
        tk.Label(self.docket_fr, text='ACTIVE DOCKET (SYSTEM FAULTS)', font=FONT['h3'], fg=PAL['text_primary'], bg=PAL['surface']).pack(anchor='w', pady=(0, 20))
        cols = ('ID', 'SHARD', 'JURISDICTION', 'SEVERITY', 'STATUS', 'ASSIGNED')
        self.tree = ttk.Treeview(self.docket_fr, columns=cols, show='headings', style='Custom.Treeview')
        for col in cols:
            self.tree.heading(col, text=col)
            self.tree.column(col, anchor='center')
        self.tree.pack(side='left', fill='both', expand=True)
        sb = ttk.Scrollbar(self.docket_fr, orient='vertical', command=self.tree.yview)
        self.tree.configure(yscrollcommand=sb.set)
        sb.pack(side='right', fill='y')

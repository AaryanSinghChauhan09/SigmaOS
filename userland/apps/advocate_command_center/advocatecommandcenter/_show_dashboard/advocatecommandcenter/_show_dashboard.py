# Generated method: AdvocateCommandCenter._show_dashboard
import tkinter as tk
from tkinter import ttk, messagebox
from typing import Dict, Any, List, Optional, Union, Callable
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT

class AdvocateCommandCenter:
    def _show_dashboard(self):
        self.stats_fr = tk.Frame(self.content_fr, bg=PAL['background'])
        self.stats_fr.pack(fill='x', pady=(0, 30))
        metrics = [('ACTIVE CASES', '24'), ('HEARINGS TODAY', '3'), ('PENDING DRAFTS', '12'), ('BILLABLES (MTD)', '₹2.4L')]
        for lbl, val in metrics:
            f = tk.Frame(self.stats_fr, bg=PAL['surface'], padx=20, pady=20)
            f.pack(side='left', expand=True, fill='both', padx=5)
            tk.Label(f, text=lbl, font=FONT['caption'], fg=PAL['text_secondary'], bg=PAL['surface']).pack(anchor='w')
            tk.Label(f, text=val, font=FONT['h2'], fg=PAL['text_primary'], bg=PAL['surface']).pack(anchor='w')
        sched_fr = tk.Frame(self.content_fr, bg=PAL['surface'], padx=20, pady=20)
        sched_fr.pack(fill='both', expand=True)
        tk.Label(sched_fr, text='UPCOMING HEARINGS (COMPLIANCE SYNC)', font=FONT['h3'], fg=PAL['secondary'], bg=PAL['surface']).pack(anchor='w', pady=(0, 15))
        cols = ('DATE', 'CASE NAME', 'COURT', 'PURPOSE', 'STATUS')
        self.hearing_tree = ttk.Treeview(sched_fr, columns=cols, show='headings', style='Custom.Treeview')
        for c in cols:
            self.hearing_tree.heading(c, text=c)
            self.hearing_tree.column(c, anchor='center')
        self.hearing_tree.pack(fill='both', expand=True)
"""
Auto-split from userland\apps\quantum_bi.py — QuantumBIEngine._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import math



class QuantumBIEngine:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='QUANTUM BI ENGINE', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        nav_btns = [('📁 IMPORT DATA', self._import_data), ('📺 KIOSK MODE (GECKOBOARD)', self._kiosk_mode), ('🧠 NEURAL FORECAST (TABLEAU)', self._forecast)]
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.model_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=250, padx=15, pady=15)
        self.model_fr.pack(side='left', fill='y', padx=(0, 20))
        self.model_fr.pack_propagate(False)
        tk.Label(self.model_fr, text='SEMANTIC LAYER', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        self.dims_tree = ttk.Treeview(self.model_fr, columns='Field', show='headings', style='BI.Treeview', height=6)
        self.dims_tree.heading('Field', text='DIMENSIONS (Drag)')
        self.dims_tree.pack(fill='x', pady=(0, 15))
        self.meas_tree = ttk.Treeview(self.model_fr, columns='Field', show='headings', style='BI.Treeview', height=6)
        self.meas_tree.heading('Field', text='MEASURES (Drag)')
        self.meas_tree.pack(fill='x', pady=(0, 15))
        tk.Label(self.model_fr, text='NATURAL LANGUAGE QUERY', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        self.query_entry = tk.Entry(self.model_fr, font=('Inter', 9), bg=PAL['bg'], fg=PAL['text'], insertbackground=PAL['accent'], relief='flat')
        self.query_entry.pack(fill='x', pady=5)
        self.query_entry.insert(0, "e.g., 'Show Revenue by Region 2026'")
        self.query_entry.bind('<Return>', lambda e: self._natural_query())
        self.viz_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.viz_fr.pack(side='left', fill='both', expand=True)
        self.dashboard_tabs = ttk.Notebook(self.viz_fr)
        self.dashboard_tabs.pack(fill='both', expand=True)
        self.canvas_fr = tk.Frame(self.dashboard_tabs, bg=PAL['bg'])
        self.dashboard_tabs.add(self.canvas_fr, text=' VISUAL RENDER ')
        self.canvas = tk.Canvas(self.canvas_fr, bg=PAL['sidebar'], highlightthickness=0)
        self.canvas.pack(fill='both', expand=True, padx=5, pady=5)
        self.canvas.create_text(350, 250, text='DRAG DIMENSIONS TO ENGINES\nOR IMPORT DATA', fill=PAL['dim'], font=('Inter', 14, 'bold'), justify='center')
        self.tab_fr = tk.Frame(self.dashboard_tabs, bg=PAL['bg'])
        self.dashboard_tabs.add(self.tab_fr, text=' SPREADSHEET MATRIX ')
        cols = ('ID', 'Region', 'Category', 'Revenue ($)', 'Growth (%)')
        self.grid = ttk.Treeview(self.tab_fr, columns=cols, show='headings', style='BI.Treeview')
        for c in cols:
            self.grid.heading(c, text=c)
            self.grid.column(c, width=120, anchor='center')
        self.grid.pack(fill='both', expand=True, padx=5, pady=5)
        self.status = tk.Label(self, text='GPU ACCELERATION: ACTIVE | SQL CONN: IDLE | DAX-COMPATIBLE', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')

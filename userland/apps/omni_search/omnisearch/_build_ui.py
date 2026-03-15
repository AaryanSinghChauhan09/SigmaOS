# Generated method: OmniSearch._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import time

class OmniSearch:
    def _build_ui(self):
        self.bar_fr = tk.Frame(self, bg=PAL['bg'], pady=30, padx=50)
        self.bar_fr.pack(side='top', fill='x')
        self.search_entry = tk.Entry(self.bar_fr, font=('Inter', 24, 'bold'), bg=PAL['panel'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat', justify='center')
        self.search_entry.pack(fill='x', ipady=15)
        self.search_entry.insert(0, 'Initiate Neural Search Sequence...')
        self.search_entry.bind('<FocusIn>', lambda e: self.search_entry.delete(0, tk.END) if self.search_entry.get() == 'Initiate Neural Search Sequence...' else None)
        self.search_entry.bind('<KeyRelease>', self._live_search)
        self.search_entry.bind('<Return>', self._execute_search)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=30, pady=10)
        self.workspace.pack(fill='both', expand=True)
        cols = ('Asset', 'Type', 'Mass', 'Vector Location')
        self.tree = ttk.Treeview(self.workspace, columns=cols, show='headings', style='Omni.Treeview', height=12)
        for c in cols:
            self.tree.heading(c, text=c.upper())
        self.tree.column('Asset', width=250, anchor='w')
        self.tree.column('Type', width=120, anchor='center')
        self.tree.column('Mass', width=80, anchor='center')
        self.tree.column('Vector Location', width=300, anchor='w')
        self.tree.pack(fill='both', expand=True)
        self.tree.bind('<Double-1>', self._launch_asset)
        for d in self.db:
            self.tree.insert('', 'end', values=d)
        self.status = tk.Label(self, text='SEMANTIC ENGINE IDLE | 0.00 MS INDEX LATENCY', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
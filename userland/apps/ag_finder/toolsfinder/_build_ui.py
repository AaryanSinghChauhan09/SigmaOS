"""
Auto-split from userland\apps\ag_finder.py — ToolsFinder._build_ui
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import os
import time



class ToolsFinder:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], padx=30, pady=25)
        head.pack(fill='x')
        tk.Label(head, text='SOVEREIGN FINDER', font=('Inter', 22, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        self.search_var = tk.StringVar()
        self.search_entry = tk.Entry(head, textvariable=self.search_var, bg='#000', fg=PAL['text'], font=('Inter', 11), borderwidth=0, insertbackground='white', width=40, highlightthickness=1, highlightbackground=PAL['border'])
        self.search_entry.pack(side='right', pady=5)
        self.search_entry.insert(0, '[ NEURAL SEARCH ]')
        self.search_entry.bind('<FocusIn>', lambda e: self.search_entry.delete(0, 'end'))
        self.search_entry.bind('<KeyRelease>', lambda e: self._filter())
        body = tk.Frame(self, bg=PAL['bg'], padx=25)
        body.pack(fill='both', expand=True)
        self.panes = ttk.PanedWindow(body, orient='horizontal')
        self.panes.pack(fill='both', expand=True)
        self.sidebar = tk.Frame(self.panes, bg=PAL['sidebar'], width=220, padx=15, pady=20)
        self.panes.add(self.sidebar, weight=1)
        self.sidebar.pack_propagate(False)
        tk.Label(self.sidebar, text='VAULTS', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar']).pack(anchor='w')
        for disk in [('💾 Main Vault', 'success'), ('🕸️ Mesh Drive', 'accent'), ('🔥 Burner Temp', 'dim')]:
            f = tk.Frame(self.sidebar, bg=PAL['sidebar'], pady=8, cursor='hand2')
            f.pack(fill='x')
            tk.Label(f, text=disk[0], font=('Inter', 10), fg=PAL['text'], bg=PAL['sidebar']).pack(side='left')
        tk.Label(self.sidebar, text='QUICK ACCESS', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar'], pady=(20, 0)).pack(anchor='w')
        for item in ['Downloads', 'Documents', 'Source_Code', 'Media']:
            tk.Label(self.sidebar, text=f'📂 {item}', font=('Inter', 9), fg=PAL['text'], bg=PAL['sidebar'], pady=8, cursor='hand2').pack(anchor='w')
        self.view_fr = tk.Frame(self.panes, bg=PAL['bg'], padx=20)
        self.panes.add(self.view_fr, weight=4)
        self.list_canvas = tk.Canvas(self.view_fr, bg=PAL['bg'], highlightthickness=0)
        self.scroll = ttk.Scrollbar(self.view_fr, orient='vertical', command=self.list_canvas.yview)
        self.grid_fr = tk.Frame(self.list_canvas, bg=PAL['bg'])
        self.list_canvas.create_window((0, 0), window=self.grid_fr, anchor='nw')
        self.list_canvas.configure(yscrollcommand=self.scroll.set)
        self.list_canvas.pack(side='left', fill='both', expand=True)
        self.scroll.pack(side='right', fill='y')
        self.grid_fr.bind('<Configure>', lambda e: self.list_canvas.configure(scrollregion=self.list_canvas.bbox('all')))
        self._filter()
        self.status = tk.Label(self, text='FINDER READY | INDEXING: 100% | WORKSPACE: SIGMA_CORE', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')

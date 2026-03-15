# Generated method: DuplicateFinder._build_ui
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import hashlib
from pathlib import Path

class DuplicateFinder:
    def _build_ui(self):
        main = tk.Frame(self, bg=PAL['bg'], padx=40, pady=40)
        main.pack(fill='both', expand=True)
        head = tk.Frame(main, bg=PAL['bg'])
        head.pack(fill='x', pady=(0, 30))
        tk.Label(head, text='DUPLICATE', font=('Inter', 22, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        tk.Label(head, text='FINDER APEX', font=('Inter', 22, 'bold'), fg='white', bg=PAL['bg']).pack(side='left', padx=5)
        self.drop = tk.Frame(main, bg=PAL['card'], height=150, highlightthickness=1, highlightbackground=PAL['border'])
        self.drop.pack(fill='x')
        self.drop.pack_propagate(False)
        self.drop_lbl = tk.Label(self.drop, text='SELECT TARGET DIRECTORY FOR FORENSIC SCAN', font=('Inter', 10), fg=PAL['dim'], bg=PAL['card'])
        self.drop_lbl.pack(expand=True)
        self.drop.bind('<Button-1>', lambda e: self._select_dir())
        self.drop_lbl.bind('<Button-1>', lambda e: self._select_dir())
        self.stats_fr = tk.Frame(main, bg=PAL['bg'], pady=20)
        self.stats_fr.pack(fill='x')
        self.progress = ttk.Progressbar(self.stats_fr, mode='determinate')
        self.progress.pack(fill='x', pady=(0, 10))
        self.stat_lbl = tk.Label(self.stats_fr, text='IDLE | AWAITING COMMAND', font=('JetBrains Mono', 8), fg=PAL['dim'], bg=PAL['bg'])
        self.stat_lbl.pack(side='left')
        self.list_fr = tk.Frame(main, bg=PAL['bg'])
        self.list_fr.pack(fill='both', expand=True)
        cols = ('file', 'path', 'size', 'hash')
        self.tree = ttk.Treeview(self.list_fr, columns=cols, show='headings', selectmode='extended')
        for col in cols:
            self.tree.heading(col, text=col.upper())
            self.tree.column(col, width=100)
        self.tree.pack(side='left', fill='both', expand=True)
        sb = ttk.Scrollbar(self.list_fr, orient='vertical', command=self.tree.yview)
        sb.pack(side='right', fill='y')
        self.tree.configure(yscrollcommand=sb.set)
        self.action_fr = tk.Frame(main, bg=PAL['bg'], pady=20)
        self.action_fr.pack(fill='x')
        tk.Button(self.action_fr, text='🚀 START FORENSIC SCAN', font=('Inter', 10, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=30, pady=12, command=self._scan).pack(side='right')
        tk.Button(self.action_fr, text='🧹 ATOMIC PURGE', font=('Inter', 10, 'bold'), bg=PAL['sidebar'] if hasattr(self, 'sidebar') else '#1C1C1E', fg=PAL['warning'], relief='flat', padx=25, pady=12, command=self._purge).pack(side='right', padx=15)
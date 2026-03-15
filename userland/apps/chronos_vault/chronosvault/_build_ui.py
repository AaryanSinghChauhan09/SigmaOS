# Generated method: ChronosVault._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import time

class ChronosVault:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='CHRONOS VAULT', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='⏳ TRIGGER TEMPORAL SNAPSHOT', font=('Inter', 9, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=15, pady=8, command=self._create_snapshot).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.conf_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=250, padx=20, pady=20)
        self.conf_fr.pack(side='left', fill='y', padx=(0, 20))
        self.conf_fr.pack_propagate(False)
        tk.Label(self.conf_fr, text='STATE METRICS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 20))
        metrics = [('Total Vault Mass:', '108.3 GB', PAL['accent']), ('Delta Efficiency:', '94.2%', PAL['success']), ('Next Auto-Anchor:', 'In 4 hrs', PAL['text'])]
        for label, val, color in metrics:
            tk.Label(self.conf_fr, text=label, font=('Inter', 9), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
            tk.Label(self.conf_fr, text=val, font=('Inter', 14, 'bold'), fg=color, bg=PAL['panel']).pack(anchor='w', pady=(2, 15))
        self.tree_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.tree_fr.pack(side='left', fill='both', expand=True)
        cols = ('State Name', 'Temporal Stamp', 'Type', 'Mass')
        self.tree = ttk.Treeview(self.tree_fr, columns=cols, show='headings', style='Chronos.Treeview', height=12)
        for c, w in zip(cols, [200, 150, 100, 100]):
            self.tree.heading(c, text=c.upper())
            self.tree.column(c, width=w, anchor='w' if c == 'State Name' else 'center')
        for item in self.snapshots:
            self.tree.insert('', 'end', values=item)
        self.tree.pack(fill='both', expand=True)
        self.tree.bind('<Double-1>', self._restore_snapshot)
        self.status = tk.Label(self, text='CHRONOS CORE ACTIVE | IMMUTABLE BLOCKCHAIN SECURED', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
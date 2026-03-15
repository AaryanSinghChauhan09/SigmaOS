"""
Auto-split from userland\apps\package_weaver.py — PackageWeaver._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class PackageWeaver:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='SOVEREIGN WEAVER DECENTRALIZED REPO', font=('Inter', 18, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        nav_btns = [('🔄 SYNC LEDGERS', self._sync_repos), ('⬆️ UPGRADE ALL', self._upgrade_all)]
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.repo_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=250, padx=20, pady=20)
        self.repo_fr.pack(side='left', fill='y', padx=(0, 20))
        self.repo_fr.pack_propagate(False)
        tk.Label(self.repo_fr, text='MIRROR NODES', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        nodes = [('Main Sovereign', '🟢 99.9% Uptime'), ('Community Edge', '🟡 94.2% Uptime'), ('Nightly Builds', '🔴 Experimental')]
        for n, stat in nodes:
            tf = tk.Frame(self.repo_fr, bg=PAL['panel'], pady=10)
            tf.pack(fill='x')
            tk.Label(tf, text=n, font=('Inter', 10, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(anchor='w')
            tk.Label(tf, text=stat, font=('Inter', 8), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        self.pkg_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.pkg_fr.pack(side='left', fill='both', expand=True)
        tk.Label(self.pkg_fr, text='AVAILABLE BINARIES', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        cols = ('Package Name', 'Version', 'Domain', 'Local Topology')
        self.tree = ttk.Treeview(self.pkg_fr, columns=cols, show='headings', style='Weaver.Treeview')
        for c in cols:
            self.tree.heading(c, text=c.upper())
            self.tree.column(c, width=150, anchor='w')
        self.tree.pack(fill='both', expand=True)
        self._populate_pkgs()
        tk.Button(self.pkg_fr, text='📥 INSTALL / 🗑️ REMOVE SELECTED', font=('Inter', 9, 'bold'), bg=PAL['accent'], fg='white', relief='flat', pady=10, command=self._act_on_package).pack(fill='x', pady=(20, 0))
        self.status = tk.Label(self, text='APT/HOMEBREW REPLACED | HASH VERIFICATION ACTIVE', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')

# Generated method: VaultKeep._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random

class VaultKeep:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='VAULT KEEP ZERO-TRUST', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='🔐 BIOMETRIC LOCK', font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=self._authenticate).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.conf_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=250, padx=20, pady=20)
        self.conf_fr.pack(side='left', fill='y', padx=(0, 20))
        self.conf_fr.pack_propagate(False)
        tk.Label(self.conf_fr, text='CRYPTO CONTEXT', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 20))
        metrics = [('Vault Standard:', 'AES-GCM-2048', PAL['accent']), ('Key Sharding:', '5 Nodes Active', PAL['success'])]
        for label, val, color in metrics:
            tk.Label(self.conf_fr, text=label, font=('Inter', 9), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
            tk.Label(self.conf_fr, text=val, font=('Inter', 12, 'bold'), fg=color, bg=PAL['panel']).pack(anchor='w', pady=(2, 15))
        tk.Button(self.conf_fr, text='GENERATE KEY', font=('Inter', 8, 'bold'), bg=PAL['accent'], fg='black', relief='flat', pady=6, command=self._mock_generate).pack(fill='x', pady=(20, 0))
        self.tree_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.tree_fr.pack(side='left', fill='both', expand=True)
        cols = ('Identity', 'Username', 'Cipher Data', 'Last Accessed')
        self.tree = ttk.Treeview(self.tree_fr, columns=cols, show='headings', style='Vault.Treeview', height=12)
        for c, w in zip(cols, [200, 150, 100, 100]):
            self.tree.heading(c, text=c.upper())
            self.tree.column(c, width=w, anchor='w' if c != 'Cipher Data' else 'center')
        for item in self.secrets:
            self.tree.insert('', 'end', values=item)
        self.tree.pack(fill='both', expand=True)
        self.tree.bind('<Double-1>', self._reveal_secret)
        self.status = tk.Label(self, text='VAULT LOCKED | AWAITING BIOMETRIC CLEARANCE', bg=PAL['danger'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
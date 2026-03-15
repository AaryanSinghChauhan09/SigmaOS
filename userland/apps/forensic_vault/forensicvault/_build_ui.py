# Generated method: ForensicVault._build_ui
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import os, hashlib, time, threading

class ForensicVault:
    def _build_ui(self):
        hdr = tk.Frame(self, bg=PAL['bg'], height=70)
        hdr.pack(fill='x')
        hdr.pack_propagate(False)
        tk.Label(hdr, text='☣ FORENSIC VAULT PRO', font=('Inter Bold', 20), fg=PAL['accent'], bg=PAL['bg']).pack(side='left', padx=30)
        self.stat_lbl = tk.Label(hdr, text='INTEGRITY: SECURE', font=('Inter', 10, 'bold'), fg=PAL['accent'], bg=PAL['bg'])
        self.stat_lbl.pack(side='right', padx=30)
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=20)
        left = tk.Frame(body, bg=PAL['sidebar'], width=350)
        left.pack(side='left', fill='y', padx=(0, 20))
        left.pack_propagate(False)
        tk.Label(left, text='WATCHLIST REGISTRY', font=('Inter Bold', 9), fg=PAL['dim'], bg=PAL['sidebar']).pack(pady=15, padx=20, anchor='w')
        self.tree = ttk.Treeview(left, columns=('status',), show='tree headings')
        self.tree.heading('#0', text='FILE')
        self.tree.heading('status', text='STATUS')
        self.tree.pack(fill='both', expand=True, padx=10, pady=10)
        right = tk.Frame(body, bg=PAL['bg'])
        right.pack(side='right', fill='both', expand=True)
        self.console = scrolledtext.ScrolledText(right, bg='#000', fg=PAL['accent'], font=('Cascadia Code', 9), borderwidth=0, padx=15, pady=15)
        self.console.pack(fill='both', expand=True)
        ctrl = tk.Frame(self, bg=PAL['sidebar'], height=60)
        ctrl.pack(side='bottom', fill='x')
        tk.Button(ctrl, text='INITIATE INTEGRITY SCAN', font=('Inter Bold', 10), bg=PAL['accent'], fg='black', relief='flat', padx=20, pady=10, command=self._start_scan).pack(side='left', padx=20)
        tk.Button(ctrl, text='PURGE TAMPERED DATA', font=('Inter Bold', 10), bg=PAL['critical'], fg='white', relief='flat', padx=20, pady=10, command=self._purge).pack(side='right', padx=20)
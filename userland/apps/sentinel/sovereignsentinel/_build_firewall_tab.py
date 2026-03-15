"""
Auto-split from userland\apps\sentinel.py — SovereignSentinel._build_firewall_tab
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess



class SovereignSentinel:
    def _build_firewall_tab(self):
        frame = tk.Frame(self.nb, bg=PAL['bg'])
        self.nb.add(frame, text='  🧱 Firewall  ')
        body = tk.Frame(frame, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        left = tk.Frame(body, bg=PAL['panel'], width=380, padx=20, pady=20)
        left.pack(side='left', fill='both', padx=(0, 10))
        left.pack_propagate(False)
        tk.Label(left, text='⚙ ADD FIREWALL RULE', font=('Segoe UI', 9, 'bold'), fg='white', bg=PAL['panel'], pady=5).pack(anchor='w')
        for label, default in [('Direction (IN/OUT/BOTH):', 'IN'), ('Protocol (TCP/UDP/ALL):', 'TCP'), ('Port or Range:', '443'), ('Action (ALLOW/BLOCK):', 'ALLOW'), ('Description:', 'HTTPS Web Traffic')]:
            tk.Label(left, text=label, font=('Segoe UI', 8), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(8, 0))
            e = ttk.Entry(left)
            e.pack(fill='x')
            e.insert(0, default)
        ttk.Button(left, text='➕ ADD RULE', command=self._add_firewall_rule).pack(fill='x', pady=15)
        ttk.Button(left, text='🔒 LOCKDOWN MODE', command=self._lockdown).pack(fill='x')
        right = tk.Frame(body, bg=PAL['bg'])
        right.pack(side='left', fill='both', expand=True)
        tk.Label(right, text='ACTIVE RULES', font=('Segoe UI', 9, 'bold'), fg=PAL['dim'], bg=PAL['bg'], pady=5).pack(anchor='w')
        fw_cols = ('Direction', 'Protocol', 'Port', 'Action', 'Description')
        self.fw_tree = ttk.Treeview(right, columns=fw_cols, show='headings', height=18)
        for col in fw_cols:
            self.fw_tree.heading(col, text=col)
            self.fw_tree.column(col, width=130, anchor='center')
        self.fw_tree.pack(fill='both', expand=True)
        self._populate_fw_rules()

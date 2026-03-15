"""
Auto-split from userland\apps\event_matrix.py — EventMatrix._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class EventMatrix:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='EVENT MATRIX KERNEL LOGS', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='🧹 PURGE ALL LOGS', font=('Inter', 9, 'bold'), bg=PAL['danger'], fg='white', relief='flat', padx=15, pady=8, command=self._purge_logs).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.filter_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=220, padx=15, pady=15)
        self.filter_fr.pack(side='left', fill='y', padx=(0, 20))
        self.filter_fr.pack_propagate(False)
        tk.Label(self.filter_fr, text='LOG PARADIGMS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        for filter_name in ['ALL SYSTEM EVENTS', 'CRITICAL KERNEL PANICS', 'SECURITY BREACHES', 'WARNING DIAGNOSTICS', 'APPLICATION INFO']:
            col = PAL['danger'] if 'CRITICAL' in filter_name else PAL['text']
            tk.Button(self.filter_fr, text=filter_name, font=('Inter', 8, 'bold'), bg=PAL['bg'], fg=col, relief='flat', pady=6, anchor='w', padx=10).pack(fill='x', pady=5)
        self.log_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.log_fr.pack(side='left', fill='both', expand=True)
        cols = ('Level', 'Timestamp', 'Source Node', 'Description / Payload')
        self.tree = ttk.Treeview(self.log_fr, columns=cols, show='headings', style='Event.Treeview')
        self.tree.heading('Level', text='SEV')
        self.tree.column('Level', width=80, anchor='center')
        self.tree.heading('Timestamp', text='SYS-TIME')
        self.tree.column('Timestamp', width=150, anchor='center')
        self.tree.heading('Source Node', text='PROCESS / THREAD')
        self.tree.column('Source Node', width=180, anchor='w')
        self.tree.heading('Description / Payload', text='NEURAL PAYLOAD')
        self.tree.column('Description / Payload', width=500, anchor='w')
        self.tree.pack(fill='both', expand=True)
        self.tree.bind('<Double-1>', self._inspect_log)
        self.status = tk.Label(self, text='JOURNALCTL STREAM ACTIVE | WRITING SECURE LOGS TO NVME', bg=PAL['accent_dim'], fg='black', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')

"""
Auto-split from userland\apps\omni_tweak_daemon.py — OmniTweakDaemon._build_daemon_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class OmniTweakDaemon:
    def _build_daemon_tab(self):
        tk.Label(self.tab_daemon, text='NEURAL DAEMON MANAGEMENT', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        top_fr = tk.Frame(self.tab_daemon, bg=PAL['bg'])
        top_fr.pack(fill='x', pady=(0, 15))
        tk.Button(top_fr, text='➕ CREATE CRON-JOB', font=('Inter', 8, 'bold'), bg=PAL['panel'], fg=PAL['text'], relief='flat', padx=10, pady=5, command=self._mock_cron).pack(side='left', padx=(0, 10))
        tk.Button(top_fr, text='⚙️ RELOAD DAEMONS (systemctl daemon-reload)', font=('Inter', 8, 'bold'), bg=PAL['panel'], fg=PAL['text'], relief='flat', padx=10, pady=5, command=self._sys_reload).pack(side='left')
        cols = ('UNIT FILE', 'LOAD', 'ACTIVE', 'SUB-STATE', 'DESCRIPTION')
        self.daemon_tree = ttk.Treeview(self.tab_daemon, columns=cols, show='headings', style='Tweak.Treeview', height=12)
        widths = [180, 80, 80, 100, 300]
        for c, w in zip(cols, widths):
            self.daemon_tree.heading(c, text=c)
            self.daemon_tree.column(c, width=w, anchor='w')
        services = [('sshd.service', 'loaded', 'active', 'running', 'OpenSSH Daemon (Encrypted)'), ('docker.socket', 'loaded', 'active', 'listening', 'Docker Unix / Quantum Socket'), ('bluetooth.target', 'loaded', 'active', 'plugged', 'Bluetooth PnP Stack'), ('cron_neural.service', 'loaded', 'active', 'running', 'Sovereign AI Task Scheduler'), ('nginx.service', 'loaded', 'inactive', 'dead', 'High-Performance Web Server')]
        for s in services:
            self.daemon_tree.insert('', 'end', values=s)
        self.daemon_tree.pack(fill='both', expand=True)
        self.daemon_tree.bind('<Double-1>', self._toggle_service)

"""
Auto-split from userland\apps\startup_orchestrator.py — StartupOrchestrator._build_sequence_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random



class StartupOrchestrator:
    def _build_sequence_tab(self):
        tk.Label(self.tab_seq, text='INIT SEQUENCE (drag to reorder — usurps rc.local & systemd ordering)', font=('Inter', 11, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        cols = ('Order', 'Unit Name', 'Type', 'Time (ms)', 'State')
        self.seq_tree = ttk.Treeview(self.tab_seq, columns=cols, show='headings', style='Boot.Treeview')
        for c, w in zip(cols, [60, 250, 100, 100, 120]):
            self.seq_tree.heading(c, text=c)
            self.seq_tree.column(c, width=w, anchor='center' if c != 'Unit Name' else 'w')
        units = [(1, 'sigma-kernel.target', 'target', 42, 'active'), (2, 'hal-init.service', 'service', 18, 'active'), (3, 'sigma-network.service', 'service', 64, 'active'), (4, 'aura-display.service', 'service', 31, 'active'), (5, 'cron_neural.service', 'service', 12, 'active'), (6, 'sigma-gui.service', 'service', 190, 'active'), (7, 'nexus-monitor.service', 'service', 9, 'active'), (8, 'ssh.service', 'service', 22, 'active')]
        for u in units:
            self.seq_tree.insert('', 'end', values=u)
        self.seq_tree.pack(fill='both', expand=True)

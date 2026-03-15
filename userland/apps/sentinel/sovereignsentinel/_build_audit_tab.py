"""
Auto-split from userland\apps\sentinel.py — SovereignSentinel._build_audit_tab
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess



class SovereignSentinel:
    def _build_audit_tab(self):
        frame = tk.Frame(self.nb, bg=PAL['bg'])
        self.nb.add(frame, text='  📋 Audit Log  ')
        ctrl = tk.Frame(frame, bg=PAL['bg'], pady=10, padx=20)
        ctrl.pack(fill='x')
        ttk.Button(ctrl, text='↻ Refresh', command=self._refresh_audit).pack(side='left')
        ttk.Button(ctrl, text='💾 Export Log', command=self._export_audit).pack(side='left', padx=10)
        cols = ('Timestamp', 'Event', 'Process', 'Severity', 'Action')
        self.audit_tree = ttk.Treeview(frame, columns=cols, show='headings', height=25)
        for col in cols:
            self.audit_tree.heading(col, text=col)
            self.audit_tree.column(col, width=180 if col in ('Event', 'Process') else 130, anchor='center')
        self.audit_tree.pack(fill='both', expand=True, padx=20)
        self._refresh_audit()

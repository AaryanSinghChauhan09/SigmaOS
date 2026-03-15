"""
Auto-split from userland\apps\sentinel.py — SovereignSentinel._build_scanner_tab
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess



class SovereignSentinel:
    def _build_scanner_tab(self):
        frame = tk.Frame(self.nb, bg=PAL['bg'])
        self.nb.add(frame, text='  🔍 Scanner  ')
        body = tk.Frame(frame, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        left = tk.Frame(body, bg=PAL['panel'], width=360, padx=20, pady=20)
        left.pack(side='left', fill='both', padx=(0, 10))
        left.pack_propagate(False)
        tk.Label(left, text='SCAN TYPE', font=('Segoe UI', 9, 'bold'), fg='white', bg=PAL['panel']).pack(anchor='w', pady=5)
        self._scan_type = tk.StringVar(value='Full Hex-Validation')
        for opt in ['Quick Scan (RAM + Processes)', 'Full Hex-Validation', 'Zero-Trust Deep Scan', 'Forensic Autopilot']:
            ttk.Radiobutton(left, text=opt, variable=self._scan_type, value=opt).pack(anchor='w', pady=3)
        ttk.Button(left, text='⚡ INITIATE SCAN', command=self._run_scan).pack(fill='x', pady=15)
        self.scan_prog = ttk.Progressbar(left, mode='determinate')
        self.scan_prog.pack(fill='x')
        self.scan_status_lbl = tk.Label(left, text='Ready.', font=('Segoe UI', 8), fg=PAL['dim'], bg=PAL['panel'])
        self.scan_status_lbl.pack(anchor='w', pady=5)
        right = tk.Frame(body, bg=PAL['bg'])
        right.pack(side='left', fill='both', expand=True)
        tk.Label(right, text='SCAN REPORT', font=('Segoe UI', 8, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w')
        self.scan_log = scrolledtext.ScrolledText(right, bg='#050508', fg=PAL['safe'], font=('Cascadia Code', 9), borderwidth=0, padx=10, pady=10)
        self.scan_log.pack(fill='both', expand=True)
        self.scan_log.insert('1.0', '[GUARDIAN] Monitoring Aether-Mesh integrity...\n[GUARDIAN] No anomalies. System clean.')
        self.scan_log.tag_config('warn', foreground=PAL['accent'])
        self.scan_log.tag_config('err', foreground=PAL['danger'])

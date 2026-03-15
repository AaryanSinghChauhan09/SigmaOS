# Generated method: SovereignShield._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
from typing import Any
from sigma_core.ui.fluid_design import ICONS, SPINNERS

class SovereignShield:
    def _build_ui(self):
        self.header = tk.Frame(self, bg='#1A1A2E', height=80)
        self.header.pack(fill='x')
        tk.Label(self.header, text=f"{ICONS.get('warden', '🛡️')} SYSTEM HARDENED", fg='#34C759', bg='#1A1A2E', font=('Segoe UI', 16, 'bold')).pack(pady=20)
        self.scan_fr = tk.Frame(self, bg='#0D0D15', padx=30, pady=30)
        self.scan_fr.pack(fill='both', expand=True)
        self.status_lbl = tk.Label(self.scan_fr, text=f"{ICONS.get('intelligence', '🔍')} Scan required. System state: ANALYZING", fg='#F2F2F7', bg='#0D0D15', font=('Segoe UI', 12))
        self.status_lbl.pack(pady=10)
        self.prog = ttk.Progressbar(self.scan_fr, orient='horizontal', length=400, mode='determinate')
        self.prog.pack(pady=20)
        ttk.Button(self.scan_fr, text=f"{ICONS.get('perf', '🚀')} DEEP SYSTEM SCAN", command=self.run_scan).pack(pady=10)
        self.rules_fr = tk.Frame(self.scan_fr, bg='#13131A', padx=10, pady=10)
        self.rules_fr.pack(fill='x', pady=20)
        tk.Label(self.rules_fr, text=f"{ICONS.get('warden', '🛡️')} ACTIVE SOVEREIGN FIREWALL RULES:", fg='#5AC8FA', bg='#13131A', font=('Segoe UI', 8)).pack(anchor='w')
        tk.Label(self.rules_fr, text='• DENY ALL INCOMING (DEFAULT)\n• ALLOW SIGMA-MESH PORT 443\n• DROP TRACKING DOMAINS (TELEMETRY)', fg='#8E8E93', bg='#13131A', justify='left').pack(anchor='w')
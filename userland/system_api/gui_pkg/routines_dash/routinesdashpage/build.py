# Generated method: RoutinesDashPage.build
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD

class RoutinesDashPage:
    def build(self):
        self.controller._build_page_header(self, 'OPENROUTINES DASHBOARD', 'Visual Automation & OS Scheduling Control')
        main_panel = tk.Frame(self, bg=PAL['bg'])
        main_panel.pack(fill='both', expand=True, padx=20, pady=10)
        routines = [('Midnight Optimization', '00:00 AM', 'Dedupe FS, Thermal Baseline recalibration'), ('Shadow Backup', '03:00 AM', 'Rsync to Sovereign Ledger'), ('Performance Warmup', '07:50 AM', 'Pre-heat Aether and Browser cache'), ('Security Scan', 'Every 2hrs', 'Registry anomaly detection (KAD)')]
        for name, time_str, desc in routines:
            c = self.controller._card(main_panel, f'🕒 {name}')
            c.master.pack(fill='x', pady=5)
            tk.Label(c, text=f'Scheduled: {time_str}', font=FONT_SMALL, fg=PAL['cyan'], bg=PAL['card']).pack(anchor='w')
            tk.Label(c, text=desc, font=('Segoe UI', 8), fg=PAL['dim'], bg=PAL['card']).pack(anchor='w')
            ttk.Button(c, text='Run Manually', command=lambda n=name: self.controller._notify('Routines', f'Executing {n} now.', 'OK')).pack(side='right', pady=(0, 10))
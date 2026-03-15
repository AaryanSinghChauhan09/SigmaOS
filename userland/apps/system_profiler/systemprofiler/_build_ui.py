# Generated method: SystemProfiler._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import platform
import random

class SystemProfiler:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=60, padx=20)
        self.header.pack(side='top', fill='x', pady=10)
        tk.Label(self.header, text='SENTINEL APEX PROFILER', font=('Inter', 18, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        nav_btns = [('⚡ REFRESH', self._force_refresh), ('🔥 OPTIMIZE', self._optimize_cores)]
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 8, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=12, pady=6, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=20, pady=10)
        self.workspace.pack(fill='both', expand=True)
        sys_f = tk.Frame(self.workspace, bg=PAL['panel'], padx=15, pady=15)
        sys_f.pack(fill='x', pady=(0, 15))
        tk.Label(sys_f, text=f'SOVEREIGN KERNEL: {platform.system()} {platform.release()} (Architecture: {platform.machine()})', font=('Inter', 10, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(anchor='w')
        tk.Label(sys_f, text=f'PROCESSOR NODE: {platform.processor()}', font=('Inter', 8), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=5)
        self.metrics_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.metrics_fr.pack(fill='both', expand=True)
        self.cpu_panel = self._build_metric_panel(self.metrics_fr, 'QUANTUM PROCESSOR', 'CPU USAGE', '0%', 'Core Threading')
        self.cpu_panel.pack(side='left', fill='both', expand=True, padx=(0, 10))
        self.ram_panel = self._build_metric_panel(self.metrics_fr, 'VOLATILE MATRIX', 'RAM USAGE', '0%', 'Memory Sectors')
        self.ram_panel.pack(side='left', fill='both', expand=True, padx=(10, 0))
        self.status = tk.Label(self, text='TELEMETRY FEED ACTIVE | ENCRYPTED RING-0 ACCESS', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')
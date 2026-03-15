# Generated method: StartupOrchestrator._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random

class StartupOrchestrator:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='SOVEREIGN STARTUP ORCHESTRATOR', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='🚀 SIMULATE BOOT', font=('Inter', 9, 'bold'), bg=PAL['accent'], fg='black', relief='flat', padx=15, pady=8, command=self._simulate_boot).pack(side='left', padx=5)
        tk.Button(btn_fr, text='⚡ OPTIMIZE ORDER (AI)', font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=self._ai_optimize).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.tabs = ttk.Notebook(self.workspace, style='Boot.TNotebook')
        self.tabs.pack(fill='both', expand=True)
        self.tab_seq = tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15)
        self.tabs.add(self.tab_seq, text='🔢 BOOT SEQUENCE')
        self._build_sequence_tab()
        self.tab_grub = tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15)
        self.tabs.add(self.tab_grub, text='📀 BOOT LOADER CONFIG')
        self._build_grub_tab()
        self.tab_fstab = tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15)
        self.tabs.add(self.tab_fstab, text='💽 MOUNT POINTS (fstab)')
        self._build_fstab_tab()
        self.status = tk.Label(self, text='INIT SYSTEM ONLINE | PID 1 SOVEREIGN | BOOT TIME: 1.42s', bg=PAL['accent_dim'], fg='black', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
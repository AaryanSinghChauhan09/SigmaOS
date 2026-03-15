"""
Auto-split from userland\apps\welcome_guide.py — WelcomeAssistant._build_ui
"""

import tkinter as tk
from tkinter import ttk
import time
from typing import Any, List, Dict



class WelcomeAssistant:
    def _build_ui(self):
        self.main_fr = tk.Frame(self, bg=PAL['bg'], padx=60, pady=50)
        self.main_fr.pack(fill='both', expand=True)
        self.icon_lbl = tk.Label(self.main_fr, text=ICONS.get('bootloader', '🚀'), font=('Segoe UI Symbol', 82), bg=PAL['bg'], fg=PAL['accent'])
        self.icon_lbl.pack(pady=(20, 10))
        self.title_lbl = tk.Label(self.main_fr, text='Initializing SigmaOS...', font=('Inter Bold', 26), fg='white', bg=PAL['bg'])
        self.title_lbl.pack()
        self.desc_lbl = tk.Label(self.main_fr, text='Setting up your Sovereign workspace environment...', font=('Inter', 12), fg=PAL['dim'], bg=PAL['bg'], pady=30, wraplength=700)
        self.desc_lbl.pack()
        self.prog_var = tk.DoubleVar(value=0)
        self.prog = ttk.Progressbar(self.main_fr, style='Welcome.TProgressbar', variable=self.prog_var, length=500, mode='determinate')
        self.prog.pack(pady=20)
        self.btn_fr = tk.Frame(self, bg=PAL['bg'], pady=40)
        self.btn_fr.pack(side='bottom', fill='x', padx=60)
        self.skip_btn = tk.Button(self.btn_fr, text=f"{ICONS.get('minimalist', '✖')} SKIP SETUP", font=('Inter Bold', 8), bg=PAL['bg'], fg=PAL['dim'], relief='flat', command=self.destroy)
        self.skip_btn.pack(side='left')
        self.next_btn = tk.Button(self.btn_fr, text='INITIALIZE ➔', font=('Inter Bold', 11), bg=PAL['accent'], fg='white', relief='flat', padx=40, pady=12, command=self._next)
        self.next_btn.pack(side='right')

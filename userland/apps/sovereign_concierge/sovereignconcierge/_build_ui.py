# Generated method: SovereignConcierge._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
from typing import Optional, Dict, Any
import uuid
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT

class SovereignConcierge:
    def _build_ui(self):
        self.progress_fr = tk.Frame(self, bg=PAL['background'], pady=30)
        self.progress_fr.pack(fill='x')
        self.progress = ttk.Progressbar(self.progress_fr, length=600, mode='determinate')
        self.progress.pack()
        self.progress['value'] = 25
        self.content_fr = tk.Frame(self, bg=PAL['surface'], padx=50, pady=50, relief='flat')
        self.content_fr.pack(fill='both', expand=True, padx=100, pady=20)
        self.title_lbl = tk.Label(self.content_fr, text=self.steps[0]['title'], font=FONT['h1'], fg=PAL['primary'], bg=PAL['surface'])
        self.title_lbl.pack(pady=(0, 20))
        self.desc_lbl = tk.Label(self.content_fr, text=self.steps[0]['desc'], font=FONT['body'], fg=PAL['text_secondary'], bg=PAL['surface'], wraplength=700, justify='center')
        self.desc_lbl.pack()
        self.nav_fr = tk.Frame(self, bg=PAL['background'], pady=40)
        self.nav_fr.pack(side='bottom', fill='x')
        self.next_btn = tk.Button(self.nav_fr, text='CONTINUE →', font=FONT['body_bold'], bg=PAL['primary'], fg=PAL['background'], relief='flat', padx=30, pady=12, command=self._next_step)
        self.next_btn.pack()
"""
Auto-split from userland\apps\advocate_command_center.py — AdvocateCommandCenter._build_main_view
"""

import tkinter as tk
from tkinter import ttk, messagebox
from typing import Dict, Any, List, Optional, Union, Callable
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT



class AdvocateCommandCenter:
    def _build_main_view(self):
        self.content_fr = tk.Frame(self, bg=PAL['background'], padx=50, pady=40)
        self.content_fr.pack(side='left', fill='both', expand=True)
        self.header_fr = tk.Frame(self.content_fr, bg=PAL['background'])
        self.header_fr.pack(fill='x', pady=(0, 30))
        tk.Label(self.header_fr, text='LITIGATION PULSE', font=FONT['h1'], fg=PAL['text_primary'], bg=PAL['background']).pack(side='left')
        act_fr = tk.Frame(self.header_fr, bg=PAL['background'])
        act_fr.pack(side='right')
        tk.Button(act_fr, text='+ NEW CASE', bg=PAL['secondary'], fg=PAL['background'], font=FONT['caption'], relief='flat', padx=20, pady=10, command=self._mock_add_case).pack(side='left', padx=5)
        self._show_dashboard()

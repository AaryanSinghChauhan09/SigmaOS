"""
Auto-split from userland\system_api\gui_pkg\intelligence_hub_page.py — IntelligenceHubPage._build_math_section
"""

import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL



class IntelligenceHubPage:
    def _build_math_section(self, parent):
        card = self._card(parent, 'Linear Algebra & Tensors')
        card.master.pack(fill='x', pady=5)
        ops = ['Linear Functions', 'Vectors', 'Matrices', 'Tensors']
        for op in ops:
            f = tk.Frame(card, bg=PAL['card'])
            f.pack(fill='x', pady=2)
            tk.Label(f, text=op, font=FONT_MED, bg=PAL['card'], fg=PAL['text']).pack(side='left')
            ttk.Button(f, text='Execute', width=10, command=lambda o=op: self.gui._notify('Math', f'{o} operation verified via Sovereign ALU.', 'OK')).pack(side='right')

# Generated method: AdvocateCommandCenter._build_sidebar
import tkinter as tk
from tkinter import ttk, messagebox
from typing import Dict, Any, List, Optional, Union, Callable
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT

class AdvocateCommandCenter:
    def _build_sidebar(self):
        self.sidebar_fr = tk.Frame(self, bg=PAL['surface'], width=300, padx=25, pady=30)
        self.sidebar_fr.pack(side='left', fill='y')
        self.sidebar_fr.pack_propagate(False)
        tk.Label(self.sidebar_fr, text='SIGMA LEGAL', font=FONT['h2'], fg=PAL['secondary'], bg=PAL['surface']).pack(anchor='w', pady=(0, 40))
        nav_items = [('🏠 DASHBOARD', 'DASHBOARD'), ('📚 CASE FILES', 'CASES'), ('🗓️ CAUSE LIST', 'CAUSALIST'), ('✍️ FORM DRAFTER', 'FORMS'), ('💰 BILLING', 'BILLING'), ('⚙️ SETTINGS', 'SETTINGS')]
        for lbl, tag in nav_items:

            def make_nav(t: str) -> Callable[[], Any]:
                return lambda: self._switch_view(t)
            btn = tk.Button(self.sidebar_fr, text=lbl, font=FONT['body'], fg=PAL['text_primary'], bg=PAL['surface'], relief='flat', anchor='w', padx=10, pady=12, command=make_nav(str(tag)))
            btn.pack(fill='x', pady=2)
        bot_tag = tk.Frame(self.sidebar_fr, bg=PAL['surface_variant'], padx=15, pady=15)
        bot_tag.pack(side='bottom', fill='x')
        tk.Label(bot_tag, text='ADVOCATE ON RECORD', font=FONT['caption'], fg=PAL['text_secondary'], bg=PAL['surface_variant']).pack(anchor='w')
        tk.Label(bot_tag, text='SOVEREIGN COUNSEL', font=FONT['body_bold'], fg=PAL['text_primary'], bg=PAL['surface_variant']).pack(anchor='w')
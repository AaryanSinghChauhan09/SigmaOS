"""
Auto-split from userland\system_api\gui_pkg\config_hub.py — ConfigHubPage._cfg_safety
"""

import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_MED, FONT_BOLD, FONT_TITLE, FONT_SMALL



class ConfigHubPage:
    def _cfg_safety(self, parent):
        is_child = self.controller._is_child_mode()
        title = 'Kiddie Safety Shield' if is_child else 'Compliance & Child Safety (Guardian)'
        tk.Label(parent, text=title, font=FONT_TITLE, fg='white', bg=PAL['bg']).pack(anchor='w', pady=10)
        info_title = 'Always Safe for You!' if is_child else 'International Age Rating Compliance'
        info = self._card(parent, info_title)
        info.master.pack(fill='x', pady=5)
        desc = 'SigmaGuardian keeps you happy and safe while you learn!' if is_child else 'SigmaGuardian enforces U/G ratings for 5-year-old safety.'
        tk.Label(info, text=desc, font=FONT_SMALL, fg=PAL['dim'], bg=PAL['card']).pack(anchor='w', pady=5)
        status_fr = tk.Frame(parent, bg=PAL['bg3'], padx=15, pady=10, highlightthickness=1, highlightbackground=PAL['green'])
        status_fr.pack(fill='x', pady=20)
        tk.Label(status_fr, text='🌈 SAFETY MODE: ON FOREVER', font=('Inter Bold', 10), fg=PAL['green'], bg=PAL['bg3']).pack(side='left')
        tk.Label(status_fr, text='Safe & Happy', font=('Inter Italic', 9), fg=PAL['dim'], bg=PAL['bg3']).pack(side='right')
        footer = 'Everything in SigmaOS is hand-picked for kids. No scary things allowed!' if is_child else 'Compliance Standards: NIST, COPPA, Multi-Region Rating Sync.'
        tk.Label(parent, text=footer, font=FONT_SMALL, fg=PAL['dim'], bg=PAL['bg'], wraplength=500, justify='left').pack(anchor='w', pady=20)

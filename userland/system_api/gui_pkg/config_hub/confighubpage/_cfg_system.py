"""
Auto-split from userland\system_api\gui_pkg\config_hub.py — ConfigHubPage._cfg_system
"""

import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_MED, FONT_BOLD, FONT_TITLE, FONT_SMALL



class ConfigHubPage:
    def _cfg_system(self, parent):
        tk.Label(parent, text='System Performance & Automation', font=FONT_TITLE, fg='white', bg=PAL['bg']).pack(anchor='w', pady=10)
        ttk.Checkbutton(parent, text='Enable Sovereign Autopilot (AI System Repair)', variable=self.gui._voice_active).pack(anchor='w', pady=5)
        ttk.Checkbutton(parent, text='Ultra Performance Mode (Disable Animations)', variable=self.gui._ultra_perf).pack(anchor='w', pady=5)
        ttk.Scale(parent, from_=0, to=100).pack(fill='x', pady=20)
        tk.Label(parent, text='Energy Impact: MINIMAL', fg=PAL['teal'], bg=PAL['bg']).pack(anchor='w')

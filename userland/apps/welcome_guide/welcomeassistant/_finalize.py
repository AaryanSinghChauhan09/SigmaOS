"""
Auto-split from userland\apps\welcome_guide.py — WelcomeAssistant._finalize
"""

import tkinter as tk
from tkinter import ttk
import time
from typing import Any, List, Dict



class WelcomeAssistant:
    def _finalize(self):
        self.title_lbl.config(text='Sovereignty Established.', fg=PAL['accent'])
        self.desc_lbl.config(text='All systems operational. Zero-Trust policy enforced.\nWelcome home, Sovereign-User. Your workspace is ready.')
        self.icon_lbl.config(text=f"{ICONS.get('minimalist', '✓')}", fg='#32D74B')
        self.next_btn.config(state='disabled', text='ESTABLISHING...')
        self.update()
        self.after(2000, self.destroy)

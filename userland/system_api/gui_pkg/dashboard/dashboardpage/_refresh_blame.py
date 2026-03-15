"""
Auto-split from userland\system_api\gui_pkg\dashboard.py — DashboardPage._refresh_blame
"""

import tkinter as tk
from tkinter import ttk, scrolledtext
import random
import time
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MONO, FONT_MED, FONT_TITLE



class DashboardPage:
    def _refresh_blame(self):
        for child in self.blame_scroll.winfo_children():
            child.destroy()
        if hasattr(self.controller.kernel, 'perf'):
            blame_list = self.controller.kernel.perf.get_competitor_blame()
            if not blame_list:
                tk.Label(self.blame_scroll, text='✔ No Shims Detected.', font=FONT_SMALL, fg=PAL['green'], bg=PAL['card']).pack()
            else:
                for b in blame_list:
                    f = tk.Frame(self.blame_scroll, bg=PAL['card'])
                    f.pack(fill='x', pady=2)
                    tk.Label(f, text=f"✖ {b['name']}", font=FONT_MONO, fg=PAL['dim'], bg=PAL['card']).pack(side='left')
                    tk.Label(f, text=b['usage'], font=FONT_MONO, fg=PAL['red'], bg=PAL['card']).pack(side='right')
        self.after(5000, self._refresh_blame)

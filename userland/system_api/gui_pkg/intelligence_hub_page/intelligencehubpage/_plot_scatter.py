"""
Auto-split from userland\system_api\gui_pkg\intelligence_hub_page.py — IntelligenceHubPage._plot_scatter
"""

import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL



class IntelligenceHubPage:
    def _plot_scatter(self):
        self.plot_canvas.delete('all')
        for _ in range(50):
            x = random.randint(20, 370)
            y = random.randint(20, 160)
            self.plot_canvas.create_oval(x, y, x + 4, y + 4, fill=PAL['cyan'], outline='')
        self.gui._notify('D3.js', 'Scatter plot points distribution complete.', 'OK')

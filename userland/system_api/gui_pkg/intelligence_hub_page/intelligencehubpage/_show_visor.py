"""
Auto-split from userland\system_api\gui_pkg\intelligence_hub_page.py — IntelligenceHubPage._show_visor
"""

import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL



class IntelligenceHubPage:
    def _show_visor(self):
        self.gui._notify('TFJS Visor', 'Visor Overlay Hydrated. Monitoring Tensors...', 'INFO')
        self._plot_scatter()

"""
Auto-split from userland\system_api\gui_pkg\intelligence_hub_page.py — IntelligenceHubPage._build_history_section
"""

import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL



class IntelligenceHubPage:
    def _build_history_section(self, parent):
        card = self._card(parent, 'Evolution of Intelligence')
        card.master.pack(fill='both', expand=True, pady=5)
        console = self._console(card, height=15)
        console.pack(fill='both', expand=True)
        if self.hub:
            summary = self.hub.history.get_summary()
            if isinstance(summary, list):
                for item in summary:
                    self.gui._log(console, f"[{item['year']}] {item['event']}", 'INFO')
            else:
                self.gui._log(console, summary, 'OK')

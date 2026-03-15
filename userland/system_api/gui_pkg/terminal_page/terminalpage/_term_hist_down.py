"""
Auto-split from userland\system_api\gui_pkg\terminal_page.py — TerminalPage._term_hist_down
"""

import tkinter as tk
from tkinter import ttk
import threading
from .base_page import SigmaPage
from .styles import PAL, FONT_MONO



class TerminalPage:
    def _term_hist_down(self, e):
        if self._term_hist_idx <= 0:
            self._term_hist_idx = -1
            self._term_input.set('')
            return
        self._term_hist_idx -= 1
        self._term_input.set(self._term_history[len(self._term_history) - 1 - self._term_hist_idx])
        self._term_entry.icursor('end')

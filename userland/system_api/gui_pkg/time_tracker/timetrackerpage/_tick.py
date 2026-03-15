"""
Auto-split from userland\system_api\gui_pkg\time_tracker.py — TimeTrackerPage._tick
"""

import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_LOGO, FONT_SMALL, FONT_BOLD



class TimeTrackerPage:
    def _tick(self):
        if self._tt_running:
            self._tt_elapsed += 1
            h = self._tt_elapsed // 3600
            m = self._tt_elapsed % 3600 // 60
            s = self._tt_elapsed % 60
            self._tt_display.config(text=f'{h:02d}:{m:02d}:{s:02d}')
            self._tt_job = self.after(1000, self._tick)

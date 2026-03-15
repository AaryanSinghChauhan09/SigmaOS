"""
Auto-split from userland\system_api\gui_pkg\time_tracker.py — TimeTrackerPage.log_task
"""

import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_LOGO, FONT_SMALL, FONT_BOLD



class TimeTrackerPage:
    def log_task(self, task_name, duration_str):
        import datetime as _dt
        now = _dt.datetime.now().strftime('%H:%M:%S')
        self._tt_tree.insert('', 0, values=(task_name, duration_str, now))
        parts = duration_str.split(':')
        if len(parts) == 3:
            secs = int(parts[0]) * 3600 + int(parts[1]) * 60 + int(parts[2])
            self._tt_total_secs += secs
            th = self._tt_total_secs // 3600
            tm = self._tt_total_secs % 3600 // 60
            self._tt_total_lbl.config(text=f'Total Logged: {th}h {tm}m')

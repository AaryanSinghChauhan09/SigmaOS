"""
Auto-split from userland\system_api\gui_pkg\time_tracker.py — TimeTrackerPage.stop_timer
"""

import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_LOGO, FONT_SMALL, FONT_BOLD



class TimeTrackerPage:
    def stop_timer(self):
        self._tt_running = False
        if self._tt_job:
            self.gui.after_cancel(self._tt_job)
        h = self._tt_elapsed // 3600
        m = self._tt_elapsed % 3600 // 60
        s = self._tt_elapsed % 60
        time_str = f'{h:02d}:{m:02d}:{s:02d}'
        task_name = self.task_var.get()
        self.log_task(task_name, time_str)
        self._tt_elapsed = 0
        self._tt_display.config(text='00:00:00')
        self.start_btn.config(text='▶ Start', style='TButton')

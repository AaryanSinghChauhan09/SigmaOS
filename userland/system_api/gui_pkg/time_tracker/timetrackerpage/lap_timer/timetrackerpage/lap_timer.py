# Generated method: TimeTrackerPage.lap_timer
import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_LOGO, FONT_SMALL, FONT_BOLD

class TimeTrackerPage:
    def lap_timer(self):
        if self._tt_elapsed > 0:
            h = self._tt_elapsed // 3600
            m = self._tt_elapsed % 3600 // 60
            s = self._tt_elapsed % 60
            self.log_task(f'[LAP] {self.task_var.get()}', f'{h:02d}:{m:02d}:{s:02d}')
# Generated method: TimeTrackerPage.start_timer
import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_LOGO, FONT_SMALL, FONT_BOLD

class TimeTrackerPage:
    def start_timer(self):
        if not self._tt_running:
            self._tt_running = True
            self._tt_task_lbl.config(text=f'Task: {self.task_var.get()}')
            self.start_btn.config(text='⏸ Pause', style='Accent.TButton')
            self._tick()
        else:
            self._tt_running = False
            if self._tt_job:
                self.gui.after_cancel(self._tt_job)
            self.start_btn.config(text='▶ Resume', style='TButton')
# Generated method: SpotItGame._tick
import tkinter as tk
from tkinter import messagebox
import random
import time
import math

class SpotItGame:
    def _tick(self):
        if not self.running:
            return
        level = LEVELS[self.level_var.get()]
        total = level['time']
        pct = self.time_left / total
        col = PAL['success'] if pct > 0.5 else PAL['warning'] if pct > 0.25 else PAL['danger']
        self.lbl_timer.config(text=str(self.time_left), fg=col)
        self.timer_bar.delete('all')
        bar_w = int(140 * pct)
        self.timer_bar.create_rectangle(0, 0, 140, 8, fill=PAL['panel'], outline='')
        if bar_w > 0:
            self.timer_bar.create_rectangle(0, 0, bar_w, 8, fill=col, outline='')
        self.time_left -= 1
        if self.time_left < 0:
            self._time_up()
        else:
            self._tick_id = self.after(1000, self._tick)
"""
Auto-split from userland\apps\spot_it.py — SpotItGame._time_up
"""

import tkinter as tk
from tkinter import messagebox
import random
import time
import math



class SpotItGame:
    def _time_up(self):
        self.running = False
        self.combo = 0
        self._update_stats()
        self.canvas.create_rectangle(0, 0, self.CANVAS_W, self.CANVAS_H, fill=PAL['danger'], stipple='gray25', outline='', tags='flash')
        self.canvas.create_text(self.CANVAS_W // 2, self.CANVAS_H // 2, text="⏰  TIME'S UP!", font=('Segoe UI', 26, 'bold'), fill='white', tags='flash')
        self.status.config(text="⏰ Time's up! Press ▶ NEW ROUND.", bg=PAL['danger'])
        self.after(1500, self._round_end)

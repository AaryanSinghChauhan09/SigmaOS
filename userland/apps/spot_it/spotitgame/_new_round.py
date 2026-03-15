"""
Auto-split from userland\apps\spot_it.py — SpotItGame._new_round
"""

import tkinter as tk
from tkinter import messagebox
import random
import time
import math



class SpotItGame:
    def _new_round(self):
        level = LEVELS[self.level_var.get()]
        self.round_n += 1
        self.time_left = level['time']
        self.running = True
        self.items = []
        self.lbl_round.config(text=str(self.round_n))
        shape_name, shape_fn = random.choice(SHAPES)
        color = random.choice(COLORS)
        self.target = (shape_name, shape_fn, color)
        self.tgt_canvas.delete('all')
        shape_fn(self.tgt_canvas, 60, 60, 30, color)
        self.lbl_tgt_name.config(text=f'{shape_name.capitalize()}')
        self._populate_canvas(level)
        if self._tick_id:
            self.after_cancel(self._tick_id)
        self._tick()
        self.btn_play.config(state='disabled')
        self.status.config(text=f"Level: {level['name']} | Find the {shape_name}!", bg=PAL['panel'])

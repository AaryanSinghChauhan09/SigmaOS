"""
Auto-split from userland\apps\shell_game.py — WatchTheCup._do_swaps
"""

import tkinter as tk
from tkinter import messagebox
import random
import time



class WatchTheCup:
    def _do_swaps(self, remaining, delay, cfg):
        if remaining <= 0:
            self.phase = 'guess'
            self.lbl_instr.config(text="🖱  Click a cup — where's the coin?", fg=PAL['accent'])
            self._draw_scene(show_coin=False)
            self.canvas.bind('<Button-1>', self._on_canvas_click)
            return
        a, b = random.sample([0, 1, 2], 2)
        self._animate_swap(a, b, delay, lambda: self._do_swaps(remaining - 1, delay, cfg))

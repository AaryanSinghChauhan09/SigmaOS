"""
Auto-split from userland\apps\shell_game.py — WatchTheCup._draw_scene
"""

import tkinter as tk
from tkinter import messagebox
import random
import time



class WatchTheCup:
    def _draw_scene(self, show_coin=False, highlight=None, result_cup=None):
        c = self.canvas
        c.delete('all')
        W, H = (760, 400)
        c.create_rectangle(0, 340, W, 400, fill='#1A1209', outline='')
        c.create_rectangle(0, 340, W, 346, fill='#2E1F0A', outline='')
        for i, cx in enumerate(self.cup_xs):
            cy = self.CUP_Y
            if show_coin and i == self.coin_pos:
                self._draw_coin(c, cx, cy + self.CUP_H - 8)
            c.create_oval(cx - 52, 338, cx + 52, 352, fill='#0A0A0A', outline='')
            col = PAL['cup_hi'] if i == highlight else PAL['cup']
            if result_cup is not None and i == result_cup:
                col = PAL['success'] if i == self.coin_pos else PAL['danger']
            pts = [cx - 42, cy + self.CUP_H, cx + 42, cy + self.CUP_H, cx + 30, cy, cx - 30, cy]
            c.create_polygon(pts, fill=col, outline='#5A3A1A', width=2)
            c.create_rectangle(cx - 36, cy - self.CUP_RIM, cx + 36, cy, fill='#5A3A1A', outline='#3A2010', width=1)
            c.create_oval(cx - 10, cy - self.CUP_RIM - 14, cx + 10, cy - self.CUP_RIM + 2, fill='#3A2010', outline='#1A0E05')
            c.create_text(cx, cy + self.CUP_H // 2 + 10, text=str(i + 1), fill='#FFD69E', font=('Segoe UI', 16, 'bold'))
        if result_cup is not None:
            self._draw_coin(c, self.cup_xs[self.coin_pos], self.CUP_Y + self.CUP_H - 8)

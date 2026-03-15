# Generated method: WatchTheCup._draw_coin
import tkinter as tk
from tkinter import messagebox
import random
import time

class WatchTheCup:
    def _draw_coin(self, c, x, y):
        r = self.COIN_R
        c.create_oval(x - r, y - r, x + r, y + r, fill='#D4AF37', outline='#FFD700', width=3)
        c.create_oval(x - r + 6, y - r + 6, x + r - 6, y + r - 6, fill='#E8C840', outline='')
        c.create_text(x, y, text='₿', fill='#8B6914', font=('Segoe UI', 14, 'bold'))
"""
Auto-split from userland\apps\spot_it.py — SpotItGame._wrong
"""

import tkinter as tk
from tkinter import messagebox
import random
import time
import math



class SpotItGame:
    def _wrong(self, item_x, item_y):
        self.canvas.create_oval(item_x - self.ITEM_SIZE - 6, item_y - self.ITEM_SIZE - 6, item_x + self.ITEM_SIZE + 6, item_y + self.ITEM_SIZE + 6, fill=PAL['danger'], outline='', tags='wrong_flash')
        self.after(300, lambda: self.canvas.delete('wrong_flash'))
        self.combo = 0
        self._update_stats()
        self.status.config(text='❌ Wrong! Keep looking…', bg=PAL['danger'])

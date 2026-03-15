"""
Auto-split from userland\apps\spot_it.py — SpotItGame._on_click
"""

import tkinter as tk
from tkinter import messagebox
import random
import time
import math



class SpotItGame:
    def _on_click(self, event):
        if not self.running:
            return
        x, y = (event.x, event.y)
        for item in self.items:
            dx, dy = (x - item['x'], y - item['y'])
            if math.hypot(dx, dy) <= self.ITEM_SIZE + 4:
                if item['is_target']:
                    self._correct()
                else:
                    self._wrong(item['x'], item['y'])
                return

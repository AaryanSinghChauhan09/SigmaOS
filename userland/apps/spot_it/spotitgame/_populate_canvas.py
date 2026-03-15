"""
Auto-split from userland\apps\spot_it.py — SpotItGame._populate_canvas
"""

import tkinter as tk
from tkinter import messagebox
import random
import time
import math



class SpotItGame:
    def _populate_canvas(self, level):
        self.canvas.delete('all')
        self.items.clear()
        W, H = (self.CANVAS_W, self.CANVAS_H)
        n = level['count']
        s = self.ITEM_SIZE
        positions = self._generate_positions(n, W, H, s)
        target_idx = random.randint(0, n - 1)
        shape_name, shape_fn, color = self.target
        for i, (x, y) in enumerate(positions):
            if i == target_idx:
                chosen_shape = shape_fn
                chosen_name = shape_name
                chosen_color = color
                is_target = True
            else:
                while True:
                    other_name, other_fn = random.choice(SHAPES)
                    other_color = random.choice(COLORS)
                    if other_name != shape_name or other_color != color:
                        break
                chosen_shape = other_fn
                chosen_name = other_name
                chosen_color = other_color
                is_target = False
            chosen_shape(self.canvas, x, y, s, chosen_color)
            hit_id = self.canvas.create_oval(x - s - 4, y - s - 4, x + s + 4, y + s + 4, fill='', outline='', tags=f'item_{i}')
            self.items.append({'idx': i, 'x': x, 'y': y, 'is_target': is_target, 'hit_id': hit_id})

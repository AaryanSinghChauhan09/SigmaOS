"""
Auto-split from userland\apps\shell_game.py — WatchTheCup._on_canvas_click
"""

import tkinter as tk
from tkinter import messagebox
import random
import time



class WatchTheCup:
    def _on_canvas_click(self, event):
        if self.phase != 'guess':
            return
        self.canvas.unbind('<Button-1>')
        chosen = self._cup_at(event.x, event.y)
        if chosen is None:
            self.canvas.bind('<Button-1>', self._on_canvas_click)
            return
        self._resolve(chosen)

"""
Auto-split from userland\apps\meshtalk.py — MeshTalk._animate_health
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random
import time



class MeshTalk:
    def _animate_health(self, step):
        self.health_canvas.delete('all')
        for i in range(15):
            h = random.randint(10, 80)
            self.health_canvas.create_rectangle(i * 15, 100 - h, i * 15 + 10, 100, fill=PAL['accent'], outline='')
        self.after(200, lambda: self._animate_health(step + 1))

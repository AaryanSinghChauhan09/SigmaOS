"""
Auto-split from userland\apps\aurapaint.py — AuraPaint.pick_color
"""

import tkinter as tk
from tkinter import colorchooser, messagebox, ttk, simpledialog, filedialog
import random
import os
from typing import Any



class AuraPaint:
    def pick_color(self):
        c = colorchooser.askcolor(initialcolor=self.curr_color)[1]
        if c:
            self.curr_color = str(c)
            self.color_box.config(bg=str(c))

"""
Auto-split from userland\apps\aurapaint.py — AuraPaint.start_draw
"""

import tkinter as tk
from tkinter import colorchooser, messagebox, ttk, simpledialog, filedialog
import random
import os
from typing import Any



class AuraPaint:
    def start_draw(self, event):
        self.last_x, self.last_y = (event.x, event.y)

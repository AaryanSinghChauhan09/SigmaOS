"""
Auto-split from userland\apps\aurapaint.py — AuraPaint.set_size
"""

import tkinter as tk
from tkinter import colorchooser, messagebox, ttk, simpledialog, filedialog
import random
import os
from typing import Any



class AuraPaint:
    def set_size(self, val):
        self.brush_size = int(float(val))

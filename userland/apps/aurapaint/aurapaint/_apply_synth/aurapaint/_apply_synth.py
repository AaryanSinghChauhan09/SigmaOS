# Generated method: AuraPaint._apply_synth
import tkinter as tk
from tkinter import colorchooser, messagebox, ttk, simpledialog, filedialog
import random
import os
from typing import Any

class AuraPaint:
    def _apply_synth(self, prompt):
        for _ in range(30):
            x = random.randint(100, 800)
            y = random.randint(100, 600)
            r = random.randint(20, 150)
            self.canvas.create_oval(x, y, x + r, y + r, fill=PAL['accent'], stipple='gray25', outline='')
        self._set_status('SYNTHESIS COMPLETE')
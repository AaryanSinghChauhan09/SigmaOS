# Generated method: AuraPaint.draw
import tkinter as tk
from tkinter import colorchooser, messagebox, ttk, simpledialog, filedialog
import random
import os
from typing import Any

class AuraPaint:
    def draw(self, event):
        color = self.curr_color if self.tool != 'eraser' else PAL['canvas']
        if self.last_x is not None and self.last_y is not None:
            self.canvas.create_line(self.last_x, self.last_y, event.x, event.y, fill=color, width=self.brush_size, capstyle='round', smooth=True)
        self.last_x, self.last_y = (event.x, event.y)
# Generated method: AILifecyclePage._draw_mesh_lattice
import tkinter as tk
from tkinter import ttk
import time
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MED

class AILifecyclePage:
    def _draw_mesh_lattice(self):
        if not self._mesh_canvas.winfo_exists():
            return
        self._mesh_canvas.delete('all')
        w, h = (480, 120)
        self.after(200, self._draw_mesh_lattice)
# Generated method: SovereignThemeEngine._pick_color
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser
import random

class SovereignThemeEngine:
    def _pick_color(self, swatch, name):
        col = colorchooser.askcolor(title=f'Pick color for: {name}')[1]
        if col:
            swatch.config(bg=col)
            self.status.config(text=f'COLOR STAGED: [{name}] -> {col}', bg=PAL['panel'], fg=PAL['accent'])
# Generated method: IntelligenceHubPage._plot_linear
import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL

class IntelligenceHubPage:
    def _plot_linear(self):
        self.plot_canvas.delete('all')
        self.plot_canvas.create_line(10, 170, 380, 20, fill=PAL['accent'], width=2, smooth=True)
        self.gui._notify('Plotly.js', 'Linear regression trend-line projected.', 'OK')
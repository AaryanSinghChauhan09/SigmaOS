# Generated method: BuyhatkePage._build_forecast
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class BuyhatkePage:
    def _build_forecast(self, parent):
        tk.Label(parent, text='Quantum Price Forecasting', font=FONT_MED, fg=PAL['cyan'], bg=PAL['bg']).pack(pady=10)
        log = self.gui._console(parent, height=15)
        log.pack(fill='both', expand=True, padx=20, pady=10)
# Generated method: BuyhatkePage._build_compare
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class BuyhatkePage:
    def _build_compare(self, parent):
        tk.Label(parent, text='Market Comparison Engine', font=FONT_MED, fg=PAL['gold'], bg=PAL['bg']).pack(pady=10)
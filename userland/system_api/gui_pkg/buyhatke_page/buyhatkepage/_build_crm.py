# Generated method: BuyhatkePage._build_crm
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class BuyhatkePage:
    def _build_crm(self, parent):
        tk.Label(parent, text='Lead Pipeline Management', font=FONT_MED, fg=PAL['cyan'], bg=PAL['bg']).pack(pady=10)
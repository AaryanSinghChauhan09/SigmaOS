import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL

class AccessPage(SigmaPage):
    """♿ Omni Access: Accessibility & Voice Command"""
    def __init__(self, parent, gui):
        super().__init__(parent, gui, "Omni Access", "Accessibility & Voice Command")
        self._build_ui()

    def _build_ui(self):
        ttk.Button(self, text="Enable Voice Nav").pack(pady=10)
        tk.Label(self, text="High Contrast: [OFF]", bg=PAL["bg"], fg=PAL["dim"]).pack()

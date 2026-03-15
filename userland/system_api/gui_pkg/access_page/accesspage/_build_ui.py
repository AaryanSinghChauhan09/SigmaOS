# Generated method: AccessPage._build_ui
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL

class AccessPage:
    def _build_ui(self):
        ttk.Button(self, text='Enable Voice Nav').pack(pady=10)
        tk.Label(self, text='High Contrast: [OFF]', bg=PAL['bg'], fg=PAL['dim']).pack()
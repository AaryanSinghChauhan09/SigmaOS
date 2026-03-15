# Generated method: GuideApex._setup_styles
import tkinter as tk
from tkinter import ttk, scrolledtext
import time

class GuideApex:
    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use('clam')
        s.configure('TNotebook', background=PAL['bg'], borderwidth=0)
        s.configure('TNotebook.Tab', background=PAL['card'], foreground=PAL['dim'], padding=[15, 8])
        s.map('TNotebook.Tab', background=[('selected', PAL['accent'])], foreground=[('selected', 'white')])
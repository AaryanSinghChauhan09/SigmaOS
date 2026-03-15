# Generated method: IndentFlow._setup_styles
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random

class IndentFlow:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Flow.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('Flow.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[15, 8], font=('Inter', 9, 'bold'))
        style.map('Flow.TNotebook.Tab', background=[('selected', PAL['accent'])])
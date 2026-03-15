# Generated method: SovereignWriter._setup_styles
import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import random
import os

class SovereignWriter:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Writer.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('Writer.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[20, 10], font=('Inter', 9, 'bold'))
        style.map('Writer.TNotebook.Tab', background=[('selected', PAL['accent'])])
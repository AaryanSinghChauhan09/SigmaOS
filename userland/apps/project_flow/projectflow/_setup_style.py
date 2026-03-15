# Generated method: ProjectFlow._setup_style
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class ProjectFlow:
    def _setup_style(self):
        s = ttk.Style()
        s.theme_use('clam')
        s.configure('PF.TNotebook', background=PAL['bg'], borderwidth=0)
        s.configure('PF.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[25, 12], font=('Inter', 9, 'bold'))
        s.map('PF.TNotebook.Tab', background=[('selected', PAL['accent'])])
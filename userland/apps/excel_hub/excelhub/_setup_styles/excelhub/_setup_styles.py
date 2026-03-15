# Generated method: ExcelHub._setup_styles
import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import random
import time

class ExcelHub:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Excel.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('Excel.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[15, 8], font=('Inter', 9, 'bold'))
        style.map('Excel.TNotebook.Tab', background=[('selected', PAL['accent'])])
        style.configure('Treeview', background=PAL['panel'], foreground=PAL['text'], fieldbackground=PAL['panel'], borderwidth=0, font=('Inter', 9))
        style.map('Treeview', background=[('selected', PAL['accent'])])
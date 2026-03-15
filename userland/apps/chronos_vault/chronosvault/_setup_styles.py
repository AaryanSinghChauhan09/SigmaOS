# Generated method: ChronosVault._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import time

class ChronosVault:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Chronos.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Inter', 10), rowheight=35)
        style.configure('Chronos.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.map('Chronos.Treeview', background=[('selected', PAL['accent_dim'])])
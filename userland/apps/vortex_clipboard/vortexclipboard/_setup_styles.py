# Generated method: VortexClipboard._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import time

class VortexClipboard:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Clip.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Inter', 9))
        style.configure('Clip.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.map('Clip.Treeview', background=[('selected', PAL['accent_dim'])])
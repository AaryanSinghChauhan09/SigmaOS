# Generated method: OmniSearch._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import time

class OmniSearch:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Omni.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Inter', 10), rowheight=30)
        style.configure('Omni.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.map('Omni.Treeview', background=[('selected', PAL['highlight'])])
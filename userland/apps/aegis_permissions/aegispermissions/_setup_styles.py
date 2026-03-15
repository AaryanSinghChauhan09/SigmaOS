# Generated method: AegisPermissions._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox

class AegisPermissions:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Aegis.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('Aegis.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[15, 8], font=('Inter', 9, 'bold'))
        style.map('Aegis.TNotebook.Tab', background=[('selected', PAL['accent'])])
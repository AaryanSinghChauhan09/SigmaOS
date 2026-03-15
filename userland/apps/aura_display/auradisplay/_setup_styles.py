# Generated method: AuraDisplay._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox

class AuraDisplay:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Aura.Horizontal.TScale', troughcolor=PAL['sidebar'], background=PAL['accent'], borderwidth=0)
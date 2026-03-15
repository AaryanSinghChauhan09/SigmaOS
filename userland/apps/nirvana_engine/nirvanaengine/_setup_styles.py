# Generated method: NirvanaEngine._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import time

class NirvanaEngine:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Nirvana.Horizontal.TProgressbar', background=PAL['accent'], troughcolor=PAL['sidebar'], borderwidth=0)
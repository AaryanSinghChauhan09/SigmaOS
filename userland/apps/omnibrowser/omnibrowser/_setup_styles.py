# Generated method: OmniBrowser._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox, simpledialog
import random
import time
from typing import Any, List
from sigma_core.ui.fluid_design import ICONS

class OmniBrowser:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Omni.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('Omni.TNotebook.Tab', background=PAL['toolbar'], foreground=PAL['text'], padding=[20, 10], font=('Inter', 9, 'bold'))
        style.map('Omni.TNotebook.Tab', background=[('selected', PAL['accent'])])
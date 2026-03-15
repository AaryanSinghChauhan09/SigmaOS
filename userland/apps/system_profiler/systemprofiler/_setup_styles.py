# Generated method: SystemProfiler._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import platform
import random

class SystemProfiler:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('TPB.Horizontal.TProgressbar', background=PAL['accent'], troughcolor=PAL['border'], borderwidth=0)
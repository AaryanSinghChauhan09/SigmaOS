"""
Auto-split from userland\apps\space_explorer.py — SpaceExplorer._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import os
import random



class SpaceExplorer:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('TProgressbar', background=PAL['accent'], troughcolor=PAL['border'], borderwidth=0)

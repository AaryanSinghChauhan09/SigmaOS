"""
Auto-split from userland\apps\omni_lens.py — OmniLensPro._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import time
import threading



class OmniLensPro:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Lens.TProgressbar', background=PAL['accent'], troughcolor=PAL['sidebar'], borderwidth=0)

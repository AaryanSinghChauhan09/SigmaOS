"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _setup_styles(self) -> None:
        s = ttk.Style()
        s.theme_use('clam')
        s.configure('Omni.TNotebook', background=PAL['bg'], borderwidth=0)
        s.configure('Omni.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[15, 8], font=('Inter', 9, 'bold'))
        s.map('Omni.TNotebook.Tab', background=[('selected', PAL['accent_dim'])])

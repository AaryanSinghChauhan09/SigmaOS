"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._pick_color
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _pick_color(self) -> None:
        col = colorchooser.askcolor(title='Pick a colour')
        if col[1]:
            messagebox.showinfo('Colour Picker', f'Selected: {col[1]}')

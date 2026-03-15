"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._calc_split
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _calc_split(self) -> None:
        try:
            each = float(self.split_total.get()) / int(self.split_people.get())
            self.split_result.config(text=f'Each pays: ${fmt(each)}', fg=PAL['success'])
        except Exception:
            self.split_result.config(text='Error: check inputs', fg=PAL['danger'])

"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._countdown
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _countdown(self, secs: int) -> None:
        if secs <= 0:
            self.timer_label.config(text="✅ TIME'S UP!", fg=PAL['success'])
            self.bell()
            return
        self.timer_label.config(text=f'⏳ {secs}s remaining', fg=PAL['accent'])
        self.after(1000, self._countdown, secs - 1)

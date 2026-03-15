"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._start_timer
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _start_timer(self) -> None:
        raw = self.timer_entry.get().strip()
        if not raw.isdigit():
            messagebox.showinfo('Timer', 'Enter a valid integer (seconds).')
            return
        self._countdown(int(raw))

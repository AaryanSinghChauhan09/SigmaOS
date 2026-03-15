"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._run_pomodoro
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _run_pomodoro(self, work_min: int, break_min: int) -> None:
        self.timer_label.config(text=f'🟢 Work: {work_min} min', fg=PAL['success'])
        self.after(1000, self._pom_tick, work_min * 60, work_min, break_min, True)

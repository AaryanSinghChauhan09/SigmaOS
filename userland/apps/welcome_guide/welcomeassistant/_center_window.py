"""
Auto-split from userland\apps\welcome_guide.py — WelcomeAssistant._center_window
"""

import tkinter as tk
from tkinter import ttk
import time
from typing import Any, List, Dict



class WelcomeAssistant:
    def _center_window(self):
        self.update_idletasks()
        w = self.winfo_width()
        h = self.winfo_height()
        extra_w = (self.winfo_screenwidth() - w) // 2
        extra_h = (self.winfo_screenheight() - h) // 2
        self.geometry(f'+{extra_w}+{extra_h}')

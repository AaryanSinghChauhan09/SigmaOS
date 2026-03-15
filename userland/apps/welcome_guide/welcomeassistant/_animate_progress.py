"""
Auto-split from userland\apps\welcome_guide.py — WelcomeAssistant._animate_progress
"""

import tkinter as tk
from tkinter import ttk
import time
from typing import Any, List, Dict



class WelcomeAssistant:
    def _animate_progress(self, target: float):
        curr = self.prog_var.get()
        if curr < target:
            self.prog_var.set(curr + 5)
            self.after(50, lambda: self._animate_progress(target))

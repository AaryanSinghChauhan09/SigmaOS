"""
Auto-split from userland\apps\bash.py — SovereignShell.history_up
"""

import tkinter as tk
from tkinter import scrolledtext, messagebox, ttk
import subprocess
import os
import sys
import random



class SovereignShell:
    def history_up(self, event):
        if self.history:
            self.history_idx = min(self.history_idx + 1, len(self.history) - 1)
            self._replace_input(self.history[-(self.history_idx + 1)])
        return 'break'

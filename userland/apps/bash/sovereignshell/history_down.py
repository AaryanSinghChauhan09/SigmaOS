"""
Auto-split from userland\apps\bash.py — SovereignShell.history_down
"""

import tkinter as tk
from tkinter import scrolledtext, messagebox, ttk
import subprocess
import os
import sys
import random



class SovereignShell:
    def history_down(self, event):
        if self.history_idx > 0:
            self.history_idx -= 1
            self._replace_input(self.history[-(self.history_idx + 1)])
        elif self.history_idx == 0:
            self.history_idx = -1
            self._replace_input('')
        return 'break'

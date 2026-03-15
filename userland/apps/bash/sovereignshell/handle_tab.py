"""
Auto-split from userland\apps\bash.py — SovereignShell.handle_tab
"""

import tkinter as tk
from tkinter import scrolledtext, messagebox, ttk
import subprocess
import os
import sys
import random



class SovereignShell:
    def handle_tab(self, event):
        self.status.config(text='NEURAL-AUTOCOMPLETED.', bg=PAL['success'])
        return 'break'

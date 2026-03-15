"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge.load_file
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def load_file(self):
        f = filedialog.askopenfilename(filetypes=[('Python', '*.py'), ('All Files', '*.*')])
        if f:
            self._open_path(f)

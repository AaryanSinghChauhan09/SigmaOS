"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge._load_default
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def _load_default(self):
        sample = '# SigmaOS CodeForge Apex Pro\n# Type Python code and press ▶ RUN\n\nprint("Hello from SigmaOS!")\n\nfor i in range(5):\n    print(f"  Iteration {i}: Neural cycle active.")\n'
        self.txt.insert('1.0', sample)
        self._on_key()

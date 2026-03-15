"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge._highlight
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def _highlight(self):
        data = self.txt.get('1.0', 'end')
        for name, (pattern, _) in KEYWORDS.items():
            self.txt.tag_remove(name, '1.0', 'end')
            for m in re.finditer(pattern, data, re.MULTILINE):
                self.txt.tag_add(name, f'1.0+{m.start()}c', f'1.0+{m.end()}c')

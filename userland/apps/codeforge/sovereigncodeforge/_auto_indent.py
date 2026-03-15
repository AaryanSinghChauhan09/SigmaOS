"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge._auto_indent
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def _auto_indent(self, event):
        """Auto-indent on newline."""
        content = self.txt.get('1.0', 'insert')
        lines = content.split('\n')
        current_line = lines[-1] if lines else ''
        indent = len(current_line) - len(current_line.lstrip())
        if current_line.rstrip().endswith(':'):
            indent += 4
        self.txt.insert('insert', '\n' + ' ' * indent)
        return 'break'

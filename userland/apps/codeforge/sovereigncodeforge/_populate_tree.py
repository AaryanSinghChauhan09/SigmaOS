"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge._populate_tree
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def _populate_tree(self):
        self._tree.delete(*self._tree.get_children())
        cwd = os.path.dirname(os.path.abspath(__file__))
        root_dir = os.path.dirname(cwd)
        root_id = self._tree.insert('', 'end', text=f'📁 {os.path.basename(root_dir)}', open=True)
        try:
            for f in sorted(os.listdir(cwd)):
                if f.endswith('.py'):
                    self._tree.insert(root_id, 'end', text=f'  🐍 {f}', values=[os.path.join(cwd, f)])
        except Exception:
            pass

"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge._open_path
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def _open_path(self, path):
        try:
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()
            self.txt.delete('1.0', 'end')
            self.txt.insert('1.0', content)
            self.current_file = path
            self._unsaved = False
            name = os.path.basename(path)
            self.file_lbl.config(text=path)
            self._active_tab.config(text=f'  {name}  ×')
            self.title(f'CodeForge Pro — {name}')
            self._on_key()
        except Exception as e:
            messagebox.showerror('Open Error', f'Cannot open file:\n{e}')

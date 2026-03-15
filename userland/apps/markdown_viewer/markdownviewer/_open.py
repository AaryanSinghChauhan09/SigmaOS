"""
Auto-split from userland\apps\markdown_viewer.py — MarkdownViewer._open
"""

import tkinter as tk
from tkinter import ttk, filedialog, scrolledtext
import re, os



class MarkdownViewer:
    def _open(self):
        path = filedialog.askopenfilename(filetypes=[('Markdown', '*.md *.txt'), ('All', '*.*')])
        if path:
            try:
                with open(path, encoding='utf-8') as f:
                    content = f.read()
                self._current_file = path
                self._file_lbl.config(text=os.path.basename(path))
                self._render(content)
            except Exception as e:
                tk.messagebox.showerror('Error', str(e))

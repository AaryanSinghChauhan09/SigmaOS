"""
Auto-split from userland\apps\markdown_viewer.py — MarkdownViewer._save_html
"""

import tkinter as tk
from tkinter import ttk, filedialog, scrolledtext
import re, os



class MarkdownViewer:
    def _save_html(self):
        path = filedialog.asksaveasfilename(defaultextension='.html', filetypes=[('HTML', '*.html')])
        if path:
            md = self._editor.get('1.0', 'end')
            html = f'<html><body><pre>{md}</pre></body></html>'
            with open(path, 'w', encoding='utf-8') as f:
                f.write(html)

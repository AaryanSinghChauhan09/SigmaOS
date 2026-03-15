"""
Auto-split from userland\apps\markdown_viewer.py — MarkdownViewer._insert_inline
"""

import tkinter as tk
from tkinter import ttk, filedialog, scrolledtext
import re, os



class MarkdownViewer:
    def _insert_inline(self, widget, text, base_tag=''):
        parts = re.split('(\\*\\*.*?\\*\\*|\\*.*?\\*|`.*?`)', text)
        for part in parts:
            if part.startswith('**') and part.endswith('**'):
                widget.insert('end', part[2:-2], 'bold')
            elif part.startswith('*') and part.endswith('*'):
                widget.insert('end', part[1:-1], 'italic')
            elif part.startswith('`') and part.endswith('`'):
                widget.insert('end', part[1:-1], 'code_inline')
            else:
                widget.insert('end', part, base_tag)

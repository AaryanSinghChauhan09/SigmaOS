# Generated method: MarkdownViewer._setup_tags
import tkinter as tk
from tkinter import ttk, filedialog, scrolledtext
import re, os

class MarkdownViewer:
    def _setup_tags(self):
        v = self._view
        v.tag_config('h1', foreground=PAL['h1'], font=('Segoe UI Bold', 22), spacing3=8)
        v.tag_config('h2', foreground=PAL['h2'], font=('Segoe UI Bold', 17), spacing3=6)
        v.tag_config('h3', foreground=PAL['h3'], font=('Segoe UI Bold', 14), spacing3=4)
        v.tag_config('bold', font=('Segoe UI Bold', 11))
        v.tag_config('italic', font=('Segoe UI Italic', 11))
        v.tag_config('code_inline', foreground='#E2C36A', font=('Cascadia Code', 10), background=PAL['code'])
        v.tag_config('code_block', foreground='#A8E6CF', font=('Cascadia Code', 9), background=PAL['code'], lmargin1=20, lmargin2=20, spacing1=4, spacing3=4)
        v.tag_config('bullet', lmargin1=20, lmargin2=30, spacing1=2)
        v.tag_config('quote', foreground=PAL['quote'], lmargin1=24, font=('Segoe UI Italic', 11))
        v.tag_config('hr', foreground=PAL['border'])
        v.tag_config('table_head', foreground=PAL['accent'], font=('Segoe UI Bold', 10))
        v.tag_config('table_row', foreground=PAL['text'], font=('Cascadia Code', 9))
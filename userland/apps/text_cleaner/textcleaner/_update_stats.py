"""
Auto-split from userland\apps\text_cleaner.py — TextCleaner._update_stats
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import re
import string
import random



class TextCleaner:
    def _update_stats(self, e=None):
        t = self.in_txt.get('1.0', 'end-1c')
        chars = len(t)
        words = len(t.split())
        t_type = 'RAW'
        if '<' in t and '>' in t:
            t_type = 'HTML'
        elif '{' in t:
            t_type = 'JSON'
        self.stats.config(text=f'{chars} CHARS | {words} WORDS | {t_type}')
        self.in_txt.edit_modified(False)

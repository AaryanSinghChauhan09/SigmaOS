"""
Auto-split from userland\apps\text_cleaner.py — TextCleaner._process
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import re
import string
import random



class TextCleaner:
    def _process(self):
        t = self.in_txt.get('1.0', 'end-1c')
        if self.vars['html'].get():
            t = re.sub('<[^>]+>', '', t)
        if self.vars['url'].get():
            t = re.sub('http[s]?://\\S+', '[LINK]', t)
        if self.vars['pii'].get():
            t = re.sub('\\S+@\\S+', '[EMAIL]', t)
        if self.vars['purge'].get():
            t = '\n'.join([l for l in t.split('\n') if l.strip()])
        if self.vars['coll'].get():
            t = re.sub('[ \\t]+', ' ', t)
        if self.vars['trim'].get():
            t = '\n'.join([l.strip() for l in t.split('\n')]).strip()
        if self.vars['up'].get():
            t = t.upper()
        if self.vars['low'].get():
            t = t.lower()
        if self.vars['sent'].get():
            t = '. '.join([s.strip().capitalize() for s in t.split('.') if s])
        self.out_txt.delete('1.0', 'end')
        self.out_txt.insert('1.0', t)
        self.tabs.select(1)
        self._set_status('NORMALIZATION SUCCESSFUL', PAL['success'])

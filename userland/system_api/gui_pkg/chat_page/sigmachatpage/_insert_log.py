"""
Auto-split from userland\system_api\gui_pkg\chat_page.py — SigmaChatPage._insert_log
"""

import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL



class SigmaChatPage:
    def _insert_log(self, sender, text, is_me=False):
        self.chat_log.config(state='normal')
        color = PAL['cyan'] if is_me else PAL['gold']
        prefix = f'[{sender}]: '
        self.chat_log.insert('end', prefix, 'prefix')
        self.chat_log.insert('end', f'{text}\n\n', 'body')
        self.chat_log.tag_config('prefix', foreground=color, font=FONT_BOLD)
        self.chat_log.tag_config('body', foreground=PAL['text'])
        self.chat_log.see('end')
        self.chat_log.config(state='disabled')
        if self.shred_mode.get():
            self.after(60000, lambda: self._shred_ui_message(text[:20]))

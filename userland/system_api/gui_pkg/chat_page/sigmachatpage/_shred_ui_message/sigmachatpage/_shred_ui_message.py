# Generated method: SigmaChatPage._shred_ui_message
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class SigmaChatPage:
    def _shred_ui_message(self, snippet):
        """USP: Visual Pixel Decay (Military-Grade UI feedback)."""
        import random
        chars = '!@#$%^&*()_+-=[]{}|;:,.<>?'
        self.chat_log.config(state='normal')
        content = self.chat_log.get('1.0', 'end')
        if snippet in content:
            idx = content.find(snippet)

            def _decay(step=0):
                if step > 5:
                    self.chat_log.config(state='normal')
                    self.chat_log.delete('1.0', 'end')
                    self.chat_log.insert('end', '[METADATA PURGED BY SOVEREIGN WARDEN]\n', 'shred')
                    self.chat_log.tag_config('shred', foreground=PAL['red'], font=FONT_SMALL)
                    self.chat_log.config(state='disabled')
                    return
                decayed = ''.join((random.choice(chars) for _ in range(len(snippet))))
                self.chat_log.config(state='normal')
                self.chat_log.insert('end', f'\n[DECAYING]: {decayed}', 'shred')
                self.chat_log.config(state='disabled')
                self.after(200, lambda: _decay(step + 1))
            _decay()
        self.chat_log.config(state='disabled')
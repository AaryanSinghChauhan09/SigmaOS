# Generated method: UIMixin._log
import tkinter as tk
from tkinter import scrolledtext, messagebox
from .styles import PAL, FONT_MONO, FONT_SMALL, FONT_BOLD

class UIMixin:
    def _log(self, console: scrolledtext.ScrolledText, text: str, tag='OK'):

        def _inner():
            if not console.winfo_exists():
                return
            console.configure(state='normal')
            console.insert('end', text + '\n', tag)
            console.see('end')
            console.configure(state='disabled')
        self.after(0, _inner)
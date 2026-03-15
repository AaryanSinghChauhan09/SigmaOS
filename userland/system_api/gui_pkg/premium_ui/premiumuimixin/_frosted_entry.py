# Generated method: PremiumUIMixin._frosted_entry
import tkinter as tk
from .styles import PAL, FONT_BOLD, FONT_SMALL

class PremiumUIMixin:
    def _frosted_entry(self, parent, placeholder='Enter command...'):
        """A sleek entry field with a 'glass' look."""
        container = tk.Frame(parent, bg=PAL['border'], padx=1, pady=1)
        entry = tk.Entry(container, bg=PAL['bg3'], fg=PAL['text'], font=('Segoe UI', 10), insertbackground=PAL['cyan'], relief='flat', bd=8)
        entry.pack(fill='x')

        def on_focus_in(e):
            container.config(bg=PAL['cyan'])
            if entry.get() == placeholder:
                entry.delete(0, 'end')
                entry.config(fg=PAL['text'])

        def on_focus_out(e):
            container.config(bg=PAL['border'])
            if not entry.get():
                entry.insert(0, placeholder)
                entry.config(fg=PAL['dim'])
        entry.insert(0, placeholder)
        entry.bind('<FocusIn>', on_focus_in)
        entry.bind('<FocusOut>', on_focus_out)
        entry.container = container
        return entry
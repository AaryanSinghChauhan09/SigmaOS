# Generated method: PremiumUIMixin._pulsing_button
import tkinter as tk
from .styles import PAL, FONT_BOLD, FONT_SMALL

class PremiumUIMixin:
    def _pulsing_button(self, parent, text, command, color=None):
        """A button that has a subtle 'glow' or 'pulse' on hover."""
        btn_color = color or PAL['accent']
        btn = tk.Button(parent, text=text, command=command, bg=btn_color, fg='white', font=FONT_BOLD, activebackground=PAL['accent2'], activeforeground='white', relief='flat', bd=0, padx=20, pady=8)

        def on_enter(e):
            btn.config(bg=PAL['accent2'])

        def on_leave(e):
            btn.config(bg=btn_color)
        btn.bind('<Enter>', on_enter)
        btn.bind('<Leave>', on_leave)
        return btn
# Generated method: PremiumUIMixin._premium_card
import tkinter as tk
from .styles import PAL, FONT_BOLD, FONT_SMALL

class PremiumUIMixin:
    def _premium_card(self, parent, title='', subtitle='', icon='💎', glass=True):
        """Creates a high-end card with subtle gradients and drop shadows (simulated)."""
        bg_color = PAL['bg2'] if glass else PAL['card']
        border_color = PAL['accent'] if glass else PAL['border']
        shadow = tk.Frame(parent, bg='#000000', padx=1, pady=1)
        container = tk.Frame(shadow, bg=bg_color, padx=15, pady=15)
        container.pack(fill='both', expand=True)
        container.master = shadow
        if title:
            header = tk.Frame(container, bg=bg_color)
            header.pack(fill='x', pady=(0, 5))
            tk.Label(header, text=icon, font=('Segoe UI', 12), fg=PAL['cyan'], bg=bg_color).pack(side='left', padx=(0, 10))
            title_fr = tk.Frame(header, bg=bg_color)
            title_fr.pack(side='left', fill='x')
            tk.Label(title_fr, text=title.upper(), font=FONT_BOLD, fg=PAL['text'], bg=bg_color).pack(anchor='w')
            if subtitle:
                tk.Label(title_fr, text=subtitle, font=('Segoe UI', 8), fg=PAL['dim'], bg=bg_color).pack(anchor='w')
            tk.Frame(container, bg=PAL['bg3'], height=1).pack(fill='x', pady=(10, 15))
        return container
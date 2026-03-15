# Generated method: BuyhatkePage.build
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class BuyhatkePage:
    def build(self):
        tab_bar = tk.Frame(self, bg=PAL['bg2'], height=40)
        tab_bar.pack(fill='x', pady=(0, 10))
        tab_bar.pack_propagate(False)
        tabs = [('Tracker', '📉'), ('Forecast', '🔮'), ('Logistics', '🚚'), ('Coupons', '🎟️'), ('Compare', '⚖️'), ('CRM', '💼')]
        self.container = tk.Frame(self, bg=PAL['bg'])
        self.container.pack(fill='both', expand=True)
        self.sub_pages = {}
        for name, icon in tabs:
            tk.Button(tab_bar, text=f'{icon} {name}', font=FONT_SMALL, fg=PAL['text'], bg=PAL['bg2'], bd=0, activebackground=PAL['bg'], command=lambda n=name.lower(): self._show_sub(n)).pack(side='left', padx=10, fill='y')
        self._show_sub('tracker')
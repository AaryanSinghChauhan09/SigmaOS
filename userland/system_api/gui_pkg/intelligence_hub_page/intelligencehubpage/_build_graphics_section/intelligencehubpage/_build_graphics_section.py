# Generated method: IntelligenceHubPage._build_graphics_section
import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL

class IntelligenceHubPage:
    def _build_graphics_section(self, parent):
        card = self._card(parent, 'JavaScript Graphics Engines')
        card.master.pack(fill='both', expand=True, pady=5)
        if self.hub:
            for lib, desc in self.hub.graphics.libraries.items():
                f = tk.Frame(card, bg=PAL['card'], pady=5)
                f.pack(fill='x')
                tk.Label(f, text=lib, font=FONT_BOLD, bg=PAL['card'], fg=PAL['cyan']).pack(anchor='w')
                tk.Label(f, text=desc, font=FONT_SMALL, bg=PAL['card'], fg=PAL['text']).pack(anchor='w')
        c_fr = tk.Frame(parent, bg=PAL['bg3'], height=200)
        c_fr.pack(fill='x', padx=10, pady=10)
        self.plot_canvas = tk.Canvas(c_fr, bg=PAL['bg3'], height=180, highlightthickness=0)
        self.plot_canvas.pack(fill='both', expand=True)
        tk.Label(c_fr, text='Live Plot Visualizer (Simulation)', font=('Inter Italic', 8), bg=PAL['bg3'], fg=PAL['dim']).pack()
        btn_fr = tk.Frame(parent, bg=PAL['bg'])
        btn_fr.pack(fill='x', pady=5)
        ttk.Button(btn_fr, text='Plot Linear Graph', command=self._plot_linear).pack(side='left', padx=5)
        ttk.Button(btn_fr, text='Plot Scatter Points', command=self._plot_scatter).pack(side='left', padx=5)
        ttk.Button(btn_fr, text='Clear Canvas', command=lambda: self.plot_canvas.delete('all')).pack(side='left', padx=5)
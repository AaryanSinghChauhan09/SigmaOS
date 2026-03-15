# Generated method: TitrationSim._build_ui
import tkinter as tk
from tkinter import messagebox
import random

class TitrationSim:
    def _build_ui(self):
        side = tk.Frame(self, bg=PAL['panel'], width=250)
        side.pack(side='left', fill='y')
        side.pack_propagate(False)
        tk.Label(side, text='TITRATION CONTROL', font=('Segoe UI Bold', 12), fg=PAL['accent'], bg=PAL['panel']).pack(pady=20)
        tk.Label(side, text=f'Flask: 20ml Base (Unknown M)', fg=PAL['text'], bg=PAL['panel']).pack(pady=5)
        tk.Label(side, text=f'Burette: 0.1M HCl (Acid)', fg=PAL['text'], bg=PAL['panel']).pack(pady=5)
        self.vol_lbl = tk.Label(side, text='Volume Added: 0.00 ml', font=('Consolas', 12), fg=PAL['base'], bg=PAL['panel'])
        self.vol_lbl.pack(pady=30)
        tk.Button(side, text='ADD DROP (0.1 ml)', command=self._add_drop, bg='#1A1E30', fg='white', relief='flat', padx=20, pady=10).pack(fill='x', padx=20, pady=5)
        tk.Button(side, text='FAST POUR (1.0 ml)', command=self._pour, bg='#1A1E30', fg='white', relief='flat', padx=20, pady=10).pack(fill='x', padx=20, pady=5)
        tk.Button(side, text='RESET LAB', command=self._reset, bg=PAL['acid'], fg='white', relief='flat', padx=20, pady=10).pack(fill='x', padx=20, pady=40)
        self.canvas = tk.Canvas(self, bg=PAL['bg'], highlightthickness=0)
        self.canvas.pack(side='right', fill='both', expand=True)
        self._draw_flask()
# Generated method: IntelligenceHubPage._build_ml_section
import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL

class IntelligenceHubPage:
    def _build_ml_section(self, parent):
        top = self._card(parent, 'Machine Learning Lifecycle')
        top.master.pack(fill='x', pady=5)
        tk.Label(top, text='Terminology:', font=FONT_BOLD, bg=PAL['card'], fg=PAL['cyan']).pack(anchor='w')
        if self.hub:
            for term, desc in self.hub.ml.terminology.items():
                tk.Label(top, text=f'• {term}: {desc}', font=FONT_SMALL, bg=PAL['card'], fg=PAL['text']).pack(anchor='w')
        ex_card = self._card(parent, 'ML Examples (TFJS / Brain.js Style)')
        ex_card.master.pack(fill='x', pady=5)
        if self.hub:
            ex1 = self.hub.deep_ml.get_example_1()
            ex2 = self.hub.deep_ml.get_example_2()
            for ex in [ex1, ex2]:
                f = tk.Frame(ex_card, bg=PAL['card'], pady=5)
                f.pack(fill='x')
                tk.Label(f, text=f"{ex['name']}: {ex['intro']}", font=FONT_BOLD, bg=PAL['card'], fg=PAL['cyan']).pack(anchor='w')
                tk.Label(f, text=f"Data: {ex['data']} | Model: {ex['model']}", font=FONT_SMALL, bg=PAL['card'], fg=PAL['text']).pack(anchor='w')
                ttk.Button(f, text='Simulate Training', command=lambda x=ex['name']: self._notify('Training', f'Started {x} training...', 'OK')).pack(anchor='e')
        act_card = self._card(parent, 'Operations & Recognition')
        act_card.master.pack(fill='x', pady=10)
        btn_fr = tk.Frame(act_card, bg=PAL['card'])
        btn_fr.pack(fill='x')
        ttk.Button(btn_fr, text='Perceptron Recognition', command=self._run_recognition).pack(side='left', padx=5)
        ttk.Button(btn_fr, text='TFJS Visor', command=self._show_visor).pack(side='left', padx=5)
        ttk.Button(btn_fr, text='Clustering Simulation', command=lambda: self.gui._notify('ML', 'K-Means grouping complete.', 'OK')).pack(side='left', padx=5)
        if self.hub:
            tk.Label(act_card, text='TFJS Models: ' + ', '.join(self.hub.deep_ml.tfjs_models), font=FONT_SMALL, bg=PAL['card'], fg=PAL['cyan']).pack(pady=5)
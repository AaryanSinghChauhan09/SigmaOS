# Generated method: TitrationSim.__init__
import tkinter as tk
from tkinter import messagebox
import random

class TitrationSim:
    def __init__(self):
        super().__init__()
        self.title('SigmaOS • Virtual Titration Lab')
        self.geometry('800x700')
        self.configure(bg=PAL['bg'])
        self.acid_conc = 0.1
        self.base_conc = _r(random.uniform(0.05, 0.15), 3)
        self.vol_in_flask = 20.0
        self.vol_added = 0.0
        self.is_done = False
        self.vol_lbl: tk.Label = tk.Label()
        self.canvas: tk.Canvas = tk.Canvas()
        self._build_ui()
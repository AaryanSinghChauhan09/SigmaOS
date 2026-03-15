# Generated method: ChemBalancer.__init__
import tkinter as tk
from tkinter import ttk, messagebox

class ChemBalancer:
    def __init__(self):
        super().__init__()
        self.title('SigmaOS • NCERT Chemical Balancer')
        self.geometry('800x500')
        self.configure(bg='#0B0D17')
        self.input_var = tk.StringVar(value='H2 + O2 -> H2O')
        self.res_lbl = tk.Label()
        self._build_ui()
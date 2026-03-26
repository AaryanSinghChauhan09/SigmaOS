# Generated method: NCERTGrapher.__init__
import tkinter as tk
from tkinter import ttk
import math

class NCERTGrapher:
    def __init__(self):
        super().__init__()
        self.title('SigmaOS • NCERT Dynamic Grapher')
        self.geometry('1000x700')
        self.configure(bg='#0B0D17')
        self.func_var = tk.StringVar(value='sin(x)')
        self.range_var = tk.DoubleVar(value=10.0)
        self.canvas = tk.Canvas()
        self._build_ui()
        self.plot()
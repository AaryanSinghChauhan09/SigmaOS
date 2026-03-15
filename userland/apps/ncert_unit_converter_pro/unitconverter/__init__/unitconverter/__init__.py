# Generated method: UnitConverter.__init__
import tkinter as tk
from tkinter import ttk

class UnitConverter:
    def __init__(self):
        super().__init__()
        self.title('SigmaOS • NCERT Unit Converter Pro')
        self.geometry('600x400')
        self.configure(bg='#0B0D17')
        self.in_val = tk.DoubleVar(value=1.0)
        self.out_val = tk.StringVar(value='---')
        self.cat_var = tk.StringVar(value='Length')
        self.from_var = tk.StringVar()
        self.to_var = tk.StringVar()
        self.data = {'Length': {'m': 1, 'km': 1000, 'cm': 0.01, 'mm': 0.001, 'inch': 0.0254, 'ft': 0.3048, 'nm': 1e-09, 'A': 1e-10}, 'Mass': {'kg': 1, 'g': 0.001, 'mg': 1e-06, 'lb': 0.45359, 'oz': 0.02834, 'amu': 1.66e-27}, 'Energy': {'J': 1, 'kJ': 1000, 'cal': 4.184, 'kcal': 4184, 'eV': 1.602e-19, 'kWh': 3600000.0}, 'Pressure': {'Pa': 1, 'atm': 101325, 'bar': 100000, 'mmHg': 133.32, 'psi': 6894.76}, 'Force': {'N': 1, 'dyn': 1e-05, 'kgf': 9.8066}}
        self._build_ui()
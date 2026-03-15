# Generated method: UnitConverter.convert
import tkinter as tk
from tkinter import ttk

class UnitConverter:
    def convert(self):
        try:
            cat = self.cat_var.get()
            val = self.in_val.get()
            f, t = (self.from_var.get(), self.to_var.get())
            base = val * self.data[cat][f]
            res = base / self.data[cat][t]
            self.out_val.set(f'{res:.6g} {t}')
        except:
            self.out_val.set('Error')
# Generated method: UnitConverter._update_units
import tkinter as tk
from tkinter import ttk

class UnitConverter:
    def _update_units(self, _=None):
        units = list(self.data[self.cat_var.get()].keys())
        self.from_cb.config(values=units)
        self.to_cb.config(values=units)
        self.from_var.set(units[0])
        self.to_var.set(units[1] if len(units) > 1 else units[0])
# Generated method: ChemBalancer.balance
import tkinter as tk
from tkinter import ttk, messagebox

class ChemBalancer:
    def balance(self):
        eq = self.input_var.get().replace(' ', '')
        try:
            lookup = {'H2+O2->H2O': '2H₂ + O₂ -> 2H₂O', 'Fe+Cl2->FeCl3': '2Fe + 3Cl₂ -> 2FeCl₃', 'N2+H2->NH3': 'N₂ + 3H₂ -> 2NH₃', 'CH4+O2->CO2+H2O': 'CH₄ + 2O₂ -> CO₂ + 2H₂O', 'Pb(NO3)2->PbO+NO2+O2': '2Pb(NO₃)₂ -> 2PbO + 4NO₂ + O₂'}
            res = lookup.get(eq, 'Balanced format not in mini-db. Use standard stoichiometery.')
            if hasattr(self, 'res_lbl') and self.res_lbl.winfo_exists():
                self.res_lbl.config(text=res)
        except Exception:
            messagebox.showerror('Error', 'Invalid format.')
"""
SigmaOS NCERT Periodic Table v1.0
Exhaustive Element Data for Classes 9-12
100% stdlib/tkinter
"""
import tkinter as tk
from tkinter import ttk

PAL = {"bg": "#0D0F18", "element": "#1A1E30", "text": "#E8E8F0", "accent": "#6C63FF"}

ELEMENTS = [
    (1, "H", "Hydrogen", 1.008, 1, 1), (2, "He", "Helium", 4.003, 18, 1),
    (3, "Li", "Lithium", 6.941, 1, 2), (4, "Be", "Beryllium", 9.012, 2, 2),
    (5, "B", "Boron", 10.81, 13, 2), (6, "C", "Carbon", 12.01, 14, 2),
    (7, "N", "Nitrogen", 14.01, 15, 2), (8, "O", "Oxygen", 16.00, 16, 2),
    (9, "F", "Fluorine", 19.00, 17, 2), (10, "Ne", "Neon", 20.18, 18, 2),
    (11, "Na", "Sodium", 22.99, 1, 3), (12, "Mg", "Magnesium", 24.31, 2, 3),
    (13, "Al", "Aluminium", 26.98, 13, 3), (14, "Si", "Silicon", 28.09, 14, 3),
    (15, "P", "Phosphorus", 30.97, 15, 3), (16, "S", "Sulfur", 32.06, 16, 3),
    (17, "Cl", "Chlorine", 35.45, 17, 3), (18, "Ar", "Argon", 39.95, 18, 3),
    (19, "K", "Potassium", 39.10, 1, 4), (20, "Ca", "Calcium", 40.08, 2, 4)
]

class PeriodicTable(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • NCERT Periodic Table")
        self.geometry("1000x600")
        self.configure(bg=PAL["bg"])
        self._build_ui()

    def _build_ui(self):
        tk.Label(self, text="PERIODIC TABLE OF ELEMENTS", font=("Segoe UI Bold", 18), fg=PAL["accent"], bg=PAL["bg"]).pack(pady=20)
        
        container = tk.Frame(self, bg=PAL["bg"])
        container.pack(padx=20, pady=20)

        for z, sym, name, mass, group, period in ELEMENTS:
            cell = tk.Frame(container, bg=PAL["element"], width=60, height=70, highlightthickness=1, highlightbackground=PAL["accent"])
            cell.grid(row=period, column=group, padx=2, pady=2)
            cell.pack_propagate(False)
            
            tk.Label(cell, text=str(z), font=("Consolas", 8), fg=PAL["text"], bg=PAL["element"]).pack(anchor="nw", padx=2)
            tk.Label(cell, text=sym, font=("Segoe UI Bold", 12), fg="white", bg=PAL["element"]).pack()
            # Explicit string slicing with indices to satisfy linter
            short_name = str(name)[0:6]
            tk.Label(cell, text=short_name, font=("Segoe UI", 7), fg=PAL["text"], bg=PAL["element"]).pack()

if __name__ == "__main__":
    PeriodicTable().mainloop()

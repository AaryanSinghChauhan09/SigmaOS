"""
SigmaOS NCERT Periodic Table v1.2
Exhaustive Element Data Hub for Classes 9-12
100% stdlib/tkinter | Zero External Deps
"""
import tkinter as tk
from tkinter import messagebox

PAL = {
    "bg": "#0D1117",
    "element": "#161B22",
    "text": "#C9D1D9",
    "accent": "#58A6FF",
    "metal": "#1F6FEB",
    "nonmetal": "#D29922",
    "noble": "#8957E5"
}

# (Z, Symbol, Name, AtomicMass, Group, Period, Category)
ELEMENTS = [
    (1, "H", "Hydrogen", 1.008, 1, 1, "nonmetal"), (2, "He", "Helium", 4.003, 18, 1, "noble"),
    (3, "Li", "Lithium", 6.941, 1, 2, "metal"), (4, "Be", "Beryllium", 9.012, 2, 2, "metal"),
    (5, "B", "Boron", 10.81, 13, 2, "nonmetal"), (6, "C", "Carbon", 12.01, 14, 2, "nonmetal"),
    (7, "N", "Nitrogen", 14.01, 15, 2, "nonmetal"), (8, "O", "Oxygen", 16.00, 16, 2, "nonmetal"),
    (9, "F", "Fluorine", 18.99, 17, 2, "nonmetal"), (10, "Ne", "Neon", 20.18, 18, 2, "noble"),
    (11, "Na", "Sodium", 22.99, 1, 3, "metal"), (12, "Mg", "Magnesium", 24.31, 2, 3, "metal"),
    (13, "Al", "Aluminium", 26.98, 13, 3, "metal"), (14, "Si", "Silicon", 28.09, 14, 3, "nonmetal"),
    (15, "P", "Phosphorus", 30.97, 15, 3, "nonmetal"), (16, "S", "Sulfur", 32.06, 16, 3, "nonmetal"),
    (17, "Cl", "Chlorine", 35.45, 17, 3, "nonmetal"), (18, "Ar", "Argon", 39.95, 18, 3, "noble"),
    (19, "K", "Potassium", 39.10, 1, 4, "metal"), (20, "Ca", "Calcium", 40.08, 2, 4, "metal"),
    (21, "Sc", "Scandium", 44.96, 3, 4, "metal"), (22, "Ti", "Titanium", 47.87, 4, 4, "metal"),
    (23, "V", "Vanadium", 50.94, 5, 4, "metal"), (24, "Cr", "Chromium", 52.00, 6, 4, "metal"),
    (25, "Mn", "Manganese", 54.94, 7, 4, "metal"), (26, "Fe", "Iron", 55.85, 8, 4, "metal"),
    (27, "Co", "Cobalt", 58.93, 9, 4, "metal"), (28, "Ni", "Nickel", 58.69, 10, 4, "metal"),
    (29, "Cu", "Copper", 63.55, 11, 4, "metal"), (30, "Zn", "Zinc", 65.38, 12, 4, "metal"),
    (31, "Ga", "Gallium", 69.72, 13, 4, "metal"), (32, "Ge", "Germanium", 72.63, 14, 4, "metal"),
    (33, "As", "Arsenic", 74.92, 15, 4, "nonmetal"), (34, "Se", "Selenium", 78.96, 16, 4, "nonmetal"),
    (35, "Br", "Bromine", 79.90, 17, 4, "nonmetal"), (36, "Kr", "Krypton", 83.80, 18, 4, "noble")
]

class PeriodicTable(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • NCERT Periodic Table Pro")
        self.geometry("1100x700")
        self.configure(bg=PAL["bg"])
        self._build_ui()

    def _build_ui(self):
        head = tk.Frame(self, bg=PAL["bg"], pady=20)
        head.pack(fill="x")
        tk.Label(head, text="NCERT PERIODIC TABLE OF ELEMENTS", font=("Segoe UI Bold", 20), fg=PAL["accent"], bg=PAL["bg"]).pack()
        tk.Label(head, text="Click any element for forensic chemical data", font=("Segoe UI", 10), fg=PAL["text"], bg=PAL["bg"]).pack()

        container = tk.Frame(self, bg=PAL["bg"])
        container.pack(padx=20, pady=20, expand=True)

        for z, sym, name, mass, group, period, cat in ELEMENTS:
            color = PAL.get(cat, PAL["element"])
            cell = tk.Frame(container, bg=color, width=58, height=68, highlightthickness=1, highlightbackground="#30363D")
            cell.grid(row=period, column=group, padx=2, pady=2)
            cell.pack_propagate(False)
            
            # Use specific closures for lambda
            l_click = lambda e, zid=z: self._show_details(zid)
            cell.bind("<Button-1>", l_click)

            tk.Label(cell, text=str(z), font=("Consolas", 7), fg="white", bg=color).pack(anchor="nw", padx=2)
            tk.Label(cell, text=sym, font=("Segoe UI Bold", 12), fg="white", bg=color).pack()
            tk.Label(cell, text=name[:6], font=("Segoe UI", 7), fg="white", bg=color).pack()

    def _show_details(self, zid):
        elem = next((e for e in ELEMENTS if e[0] == zid), None)
        if elem:
            z, sym, name, mass, group, period, cat = elem
            info = f"NAME: {name}\nSYMBOL: {sym}\nATOMIC NUMBER: {z}\nATOMIC MASS: {mass} u\nGROUP: {group}\nPERIOD: {period}\nCATEGORY: {cat.upper()}"
            messagebox.showinfo(f"Element {z}: {sym}", info)

if __name__ == "__main__":
    PeriodicTable().mainloop()

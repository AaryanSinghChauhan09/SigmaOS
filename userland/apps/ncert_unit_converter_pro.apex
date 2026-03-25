"""
SigmaOS NCERT Unit Converter Pro v1.0
Exhaustive Conversion Tool for Physics & Chemistry
100% stdlib/tkinter
"""
import tkinter as tk
from tkinter import ttk

class UnitConverter(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • NCERT Unit Converter Pro")
        self.geometry("600x400")
        self.configure(bg="#0B0D17")
        
        self.in_val = tk.DoubleVar(value=1.0)
        self.out_val = tk.StringVar(value="---")
        
        self.cat_var = tk.StringVar(value="Length")
        self.from_var = tk.StringVar()
        self.to_var = tk.StringVar()
        
        self.data = {
            "Length": {"m":1, "km":1000, "cm":0.01, "mm":0.001, "inch":0.0254, "ft":0.3048, "nm":1e-9, "A":1e-10},
            "Mass": {"kg":1, "g":0.001, "mg":1e-6, "lb":0.45359, "oz":0.02834, "amu":1.66e-27},
            "Energy": {"J":1, "kJ":1000, "cal":4.184, "kcal":4184, "eV":1.602e-19, "kWh":3.6e6},
            "Pressure": {"Pa":1, "atm":101325, "bar":100000, "mmHg":133.32, "psi":6894.76},
            "Force": {"N":1, "dyn":1e-5, "kgf":9.8066}
        }
        
        self._build_ui()

    def _build_ui(self):
        hdr = tk.Frame(self, bg="#11142A", height=50)
        hdr.pack(fill="x")
        tk.Label(hdr, text="📏 EXHAUSTIVE UNIT CONVERTER", fg="#EC4899", bg="#11142A", font=("Segoe UI Bold", 12)).pack(pady=10)

        main = tk.Frame(self, bg="#0B0D17", padx=30, pady=20)
        main.pack(fill="both", expand=True)

        tk.Label(main, text="Category:", fg="white", bg="#0B0D17").grid(row=0, column=0, sticky="w", pady=5)
        self.cat_cb = ttk.Combobox(main, textvariable=self.cat_var, values=list(self.data.keys()), state="readonly")
        self.cat_cb.grid(row=0, column=1, sticky="ew", pady=5)
        self.cat_cb.bind("<<ComboboxSelected>>", self._update_units)

        tk.Label(main, text="Input Value:", fg="white", bg="#0B0D17").grid(row=1, column=0, sticky="w", pady=10)
        tk.Entry(main, textvariable=self.in_val, bg="#1A1E30", fg="white", relief="flat").grid(row=1, column=1, sticky="ew")

        tk.Label(main, text="From:", fg="white", bg="#0B0D17").grid(row=2, column=0, sticky="w", pady=5)
        self.from_cb = ttk.Combobox(main, textvariable=self.from_var, state="readonly")
        self.from_cb.grid(row=2, column=1, sticky="ew", pady=5)

        tk.Label(main, text="To:", fg="white", bg="#0B0D17").grid(row=3, column=0, sticky="w", pady=5)
        self.to_cb = ttk.Combobox(main, textvariable=self.to_var, state="readonly")
        self.to_cb.grid(row=3, column=1, sticky="ew", pady=5)

        tk.Button(main, text="CONVERT", command=self.convert, bg="#6C63FF", fg="white", relief="flat", pady=8).grid(row=4, column=0, columnspan=2, sticky="ew", pady=20)

        self.res_lbl = tk.Label(main, textvariable=self.out_val, fg="#00D26A", bg="#0B0D17", font=("Segoe UI Bold", 14))
        self.res_lbl.grid(row=5, column=0, columnspan=2, pady=10)

        self._update_units()

    def _update_units(self, _=None):
        units = list(self.data[self.cat_var.get()].keys())
        self.from_cb.config(values=units)
        self.to_cb.config(values=units)
        self.from_var.set(units[0])
        self.to_var.set(units[1] if len(units)>1 else units[0])

    def convert(self):
        try:
            cat = self.cat_var.get()
            val = self.in_val.get()
            f, t = self.from_var.get(), self.to_var.get()
            
            # Convert to base unit then to target
            base = val * self.data[cat][f]
            res = base / self.data[cat][t]
            
            self.out_val.set(f"{res:.6g} {t}")
        except:
            self.out_val.set("Error")

if __name__ == "__main__":
    UnitConverter().mainloop()

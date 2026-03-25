"""
SigmaOS Universal Unit Converter v1.0
100% stdlib — zero 3rd-party deps.
"""
import tkinter as tk
from tkinter import ttk

PAL={"bg":"#0D0F18","panel":"#13162A","card":"#1A1E30","accent":"#F59E0B",
     "text":"#E8E8F0","dim":"#9090A0","border":"#2A2D45","success":"#00D26A"}

CATEGORIES = {
    "Length": {
        "Metre":1,"Kilometre":1e3,"Centimetre":1e-2,"Millimetre":1e-3,
        "Micrometre":1e-6,"Nanometre":1e-9,"Mile":1609.344,"Yard":0.9144,
        "Foot":0.3048,"Inch":0.0254,"Light-year":9.461e15,"Nautical Mile":1852,
    },
    "Mass": {
        "Kilogram":1,"Gram":1e-3,"Milligram":1e-6,"Tonne":1e3,
        "Pound":0.453592,"Ounce":0.028350,"Stone":6.35029,"Carat":2e-4,
    },
    "Temperature": {"Celsius":("C",),"Fahrenheit":("F",),"Kelvin":("K",)},
    "Area": {
        "sq metre":1,"sq kilometre":1e6,"sq centimetre":1e-4,"sq mile":2589988,
        "sq yard":0.836127,"sq foot":0.092903,"Acre":4046.86,"Hectare":10000,
    },
    "Volume": {
        "Litre":1,"Millilitre":1e-3,"Cubic metre":1e3,"Cubic cm":1e-3,
        "Gallon(US)":3.78541,"Quart":0.946353,"Pint":0.473176,"Cup":0.236588,
        "Fluid ounce":0.029574,"Tablespoon":0.014787,"Teaspoon":0.004929,
    },
    "Speed": {
        "m/s":1,"km/h":1/3.6,"mph":0.44704,"knot":0.514444,
        "ft/s":0.3048,"Mach":340.29,
    },
    "Time": {
        "Second":1,"Millisecond":1e-3,"Microsecond":1e-6,"Minute":60,
        "Hour":3600,"Day":86400,"Week":604800,"Month(30d)":2592000,"Year":31536000,
    },
    "Energy": {
        "Joule":1,"Kilojoule":1e3,"Calorie":4.184,"Kilocalorie":4184,
        "Wh":3600,"kWh":3.6e6,"eV":1.602e-19,"BTU":1055.06,
    },
    "Pressure": {
        "Pascal":1,"Kilopascal":1e3,"Megapascal":1e6,"Bar":1e5,
        "atm":101325,"psi":6894.76,"mmHg":133.322,"Torr":133.322,
    },
    "Data": {
        "Bit":1,"Byte":8,"Kilobyte":8192,"Megabyte":8_388_608,
        "Gigabyte":8_589_934_592,"Terabyte":8_796_093_022_208,
        "Kilobit":1e3,"Megabit":1e6,"Gigabit":1e9,
    },
}

def convert(category, value, from_u, to_u):
    if category == "Temperature":
        C_map = {
            ("Celsius","Fahrenheit"):    lambda v: v*9/5+32,
            ("Fahrenheit","Celsius"):    lambda v: (v-32)*5/9,
            ("Celsius","Kelvin"):        lambda v: v+273.15,
            ("Kelvin","Celsius"):        lambda v: v-273.15,
            ("Fahrenheit","Kelvin"):     lambda v: (v-32)*5/9+273.15,
            ("Kelvin","Fahrenheit"):     lambda v: (v-273.15)*9/5+32,
        }
        fn = C_map.get((from_u, to_u), lambda v: v)
        return fn(value)
    units = CATEGORIES[category]
    return value * units[from_u] / units[to_u]

class UnitConverter(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.title("SigmaOS Unit Converter"); self.geometry("600x500")
        self.configure(bg=PAL["bg"]); self.resizable(False, False)
        self._cat = tk.StringVar(value="Length")
        self._from = tk.StringVar(); self._to = tk.StringVar()
        self._build()

    def _build(self):
        hdr = tk.Frame(self, bg=PAL["panel"], height=50)
        hdr.pack(fill="x"); hdr.pack_propagate(False)
        tk.Label(hdr, text="⚖  UNIT CONVERTER", fg=PAL["accent"],
                 bg=PAL["panel"], font=("Segoe UI Bold",13)).pack(side="left",padx=18,pady=10)

        body = tk.Frame(self, bg=PAL["bg"], padx=24, pady=20)
        body.pack(fill="both", expand=True)

        # Category
        tk.Label(body, text="Category", fg=PAL["dim"], bg=PAL["bg"],
                 font=("Segoe UI",9)).grid(row=0,column=0,sticky="w",pady=4)
        cat_cb = ttk.Combobox(body, textvariable=self._cat,
                              values=list(CATEGORIES), state="readonly",
                              width=22, font=("Segoe UI",10))
        cat_cb.grid(row=0,column=1,columnspan=2,sticky="w",padx=8)
        cat_cb.bind("<<ComboboxSelected>>", self._on_cat)

        # From / To unit
        for col,lbl,var in ((0,"From",self._from),(2,"To",self._to)):
            tk.Label(body, text=lbl, fg=PAL["dim"], bg=PAL["bg"],
                     font=("Segoe UI",9)).grid(row=1,column=col,sticky="w",pady=8)
        self._from_cb = ttk.Combobox(body, textvariable=self._from,
                                     state="readonly", width=18, font=("Segoe UI",10))
        self._from_cb.grid(row=1,column=1,padx=8)
        self._to_cb = ttk.Combobox(body, textvariable=self._to,
                                   state="readonly", width=18, font=("Segoe UI",10))
        self._to_cb.grid(row=1,column=3,padx=8)

        # Value entry
        tk.Label(body, text="Value", fg=PAL["dim"], bg=PAL["bg"],
                 font=("Segoe UI",9)).grid(row=2,column=0,sticky="w",pady=8)
        self._val_entry = tk.Entry(body, bg=PAL["card"], fg="white", font=("Cascadia Code",13),
                                   insertbackground="white", relief="flat",
                                   highlightthickness=1, highlightbackground=PAL["border"], width=22)
        self._val_entry.insert(0,"1")
        self._val_entry.grid(row=2, column=1, columnspan=3, sticky="w", padx=8)
        self._val_entry.bind("<Return>", lambda e: self._run())

        # Convert button
        tk.Button(body, text="CONVERT  →", bg=PAL["accent"], fg="white",
                  font=("Segoe UI Bold",10), relief="flat", padx=24, pady=8,
                  command=self._run).grid(row=3,column=0,columnspan=4,pady=12,sticky="w")

        # Result
        self._result = tk.Label(body, text="", fg=PAL["success"], bg=PAL["bg"],
                                font=("Cascadia Code",22))
        self._result.grid(row=4,column=0,columnspan=4,sticky="w",pady=8)

        self._formula = tk.Label(body, text="", fg=PAL["dim"], bg=PAL["bg"],
                                 font=("Segoe UI",9))
        self._formula.grid(row=5,column=0,columnspan=4,sticky="w")

        # Common conversions reference
        ref = tk.Frame(body, bg=PAL["card"], padx=12, pady=8)
        ref.grid(row=6,column=0,columnspan=4,sticky="ew",pady=(20,0))
        tk.Label(ref, text="Quick References:", fg=PAL["accent"], bg=PAL["card"],
                 font=("Segoe UI Bold",9)).pack(anchor="w")
        refs=["1 mile = 1.609 km","1 kg = 2.205 lbs","0°C = 32°F = 273.15 K",
              "1 cal = 4.184 J","1 atm = 101325 Pa","1 GB = 1024 MB"]
        for r in refs:
            tk.Label(ref,text=f"  • {r}",fg=PAL["dim"],bg=PAL["card"],
                     font=("Segoe UI",8)).pack(anchor="w")

        self._on_cat()

    def _on_cat(self, *_):
        cat = self._cat.get()
        units = list(CATEGORIES[cat].keys())
        self._from_cb["values"] = units; self._to_cb["values"] = units
        self._from.set(units[0]); self._to.set(units[1] if len(units)>1 else units[0])

    def _run(self):
        try:
            val = float(self._val_entry.get())
            res = convert(self._cat.get(), val, self._from.get(), self._to.get())
            self._result.config(text=f"{round(res,8)}")
            self._formula.config(text=f"{val} {self._from.get()} = {round(res,8)} {self._to.get()}")
        except Exception as ex:
            self._result.config(text=f"Error: {ex}")

def launch(kernel=None):
    UnitConverter(kernel).mainloop()

if __name__ == "__main__":
    launch()

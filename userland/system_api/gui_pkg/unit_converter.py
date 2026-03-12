import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MED

class UnitConverterPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "UNIVERSAL UNIT CONVERTER", "Precise Measurement Conversions")

        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)
        
        # Categories
        cat_fr = tk.Frame(body, bg=PAL["bg"])
        cat_fr.pack(fill="x", pady=5)
        
        tk.Label(cat_fr, text="Category:", font=FONT_MED, bg=PAL["bg"], fg=PAL["gold"]).pack(side="left", padx=5)
        
        categories = ["Length", "Mass", "Temperature", "Speed", "Time", "Data Storage"]
        self.cat_var = tk.StringVar(value=categories[0])
        cat_cb = ttk.Combobox(cat_fr, textvariable=self.cat_var, values=categories, state="readonly", width=15)
        cat_cb.pack(side="left", padx=5)
        
        # Converter Frame
        conv_fr = self.controller._card(body, "Conversion Engine")
        conv_fr.master.pack(fill="x", pady=20)

        # Units map
        self.units = {
            "Length": ["Meters", "Kilometers", "Centimeters", "Millimeters", "Miles", "Yards", "Feet", "Inches"],
            "Mass": ["Kilograms", "Grams", "Milligrams", "Metric Tons", "Pounds", "Ounces"],
            "Temperature": ["Celsius", "Fahrenheit", "Kelvin"],
            "Speed": ["Meters per second", "Kilometers per hour", "Miles per hour", "Knots"],
            "Time": ["Seconds", "Minutes", "Hours", "Days", "Weeks", "Years"],
            "Data Storage": ["Bytes", "Kilobytes", "Megabytes", "Gigabytes", "Terabytes"]
        }
        
        input_fr = tk.Frame(conv_fr, bg=PAL["card"])
        input_fr.pack(pady=10)
        
        self.from_val = tk.StringVar(value="1")
        tk.Entry(input_fr, textvariable=self.from_val, bg=PAL["bg3"], fg="white", font=FONT_MED, width=15).pack(side="left", padx=10)
        
        self.from_unit = tk.StringVar(value=self.units["Length"][0])
        self.from_cb = ttk.Combobox(input_fr, textvariable=self.from_unit, values=self.units["Length"], state="readonly", width=20)
        self.from_cb.pack(side="left", padx=10)
        
        tk.Label(input_fr, text="=", font=FONT_BOLD, fg=PAL["teal"], bg=PAL["card"]).pack(side="left", padx=10)
        
        self.to_val = tk.StringVar(value="")
        tk.Entry(input_fr, textvariable=self.to_val, bg=PAL["bg3"], fg="white", font=FONT_MED, width=15, state="readonly").pack(side="left", padx=10)
        
        self.to_unit = tk.StringVar(value=self.units["Length"][1])
        self.to_cb = ttk.Combobox(input_fr, textvariable=self.to_unit, values=self.units["Length"], state="readonly", width=20)
        self.to_cb.pack(side="left", padx=10)
        
        def update_categories(*args):
            cat = self.cat_var.get()
            u = self.units[cat]
            self.from_cb.config(values=u)
            self.to_cb.config(values=u)
            self.from_unit.set(u[0])
            self.to_unit.set(u[1] if len(u)>1 else u[0])
            _convert()

        self.cat_var.trace_add("write", update_categories)

        def _convert(*args):
            try:
                v = float(self.from_val.get())
            except:
                self.to_val.set("ERR")
                return
            
            cat = self.cat_var.get()
            f_u = self.from_unit.get()
            t_u = self.to_unit.get()
            
            res = v
            # Very basic conversion logic
            if cat == "Length":
                factors = {"Meters": 1, "Kilometers": 1000, "Centimeters": 0.01, "Millimeters": 0.001, "Miles": 1609.34, "Yards": 0.9144, "Feet": 0.3048, "Inches": 0.0254}
                v_in_meters = v * factors.get(f_u, 1)
                res = v_in_meters / factors.get(t_u, 1)
            elif cat == "Mass":
                factors = {"Kilograms": 1, "Grams": 0.001, "Milligrams": 0.000001, "Metric Tons": 1000, "Pounds": 0.453592, "Ounces": 0.0283495}
                v_in_kg = v * factors.get(f_u, 1)
                res = v_in_kg / factors.get(t_u, 1)
            elif cat == "Temperature":
                if f_u == "Celsius" and t_u == "Fahrenheit": res = (v * 9/5) + 32
                elif f_u == "Fahrenheit" and t_u == "Celsius": res = (v - 32) * 5/9
                elif f_u == "Celsius" and t_u == "Kelvin": res = v + 273.15
                elif f_u == "Kelvin" and t_u == "Celsius": res = v - 273.15
                elif f_u == "Fahrenheit" and t_u == "Kelvin": res = (v - 32) * 5/9 + 273.15
                elif f_u == "Kelvin" and t_u == "Fahrenheit": res = (v - 273.15) * 9/5 + 32
            elif cat == "Storage":
                factors = {"Bytes": 1, "Kilobytes": 1024, "Megabytes": 1024**2, "Gigabytes": 1024**3, "Terabytes": 1024**4}
                v_in_bytes = v * factors.get(f_u, 1)
                res = v_in_bytes / factors.get(t_u, 1)
                
            self.to_val.set(f"{res:.4f}")

        self.from_val.trace_add("write", _convert)
        self.from_unit.trace_add("write", _convert)
        self.to_unit.trace_add("write", _convert)
        
        ttk.Button(conv_fr, text="Convert Now", command=_convert, style="Teal.TButton").pack(pady=10)
        
        # Log to show
        self.conv_log = self.controller._console(body, height=15)
        self.conv_log.pack(fill="both", expand=True, pady=10)
        self.controller._log(self.conv_log, "Universal Unit Converter Online.", "INFO")

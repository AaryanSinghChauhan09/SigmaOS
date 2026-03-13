"""
SigmaOS NCERT Primary Mathematics Lab v3.0
Classes 1–5 | Foundational Arithmetic, Shapes, Measurement
100% stdlib, zero 3rd-party deps
"""
import math, random

class Maths_Primary:
    TITLE = "Classes 1–5 – Arithmetic, Geometry & Logic Foundations"
    EXP_DATA = {
        "Number to Words": ("number_to_words", [("Number", "42")]),
        "Multiplication Tables": ("tables", [("Number", "7"), ("Up to", "10")]),
        "Area/Perimeter": ("area_peri", [("Length", "10"), ("Width", "5")]),
        "Money Addition": ("money_add", [("Item 1 (Rs)", "45.50"), ("Item 2 (Rs)", "20.25")]),
        "Simple Patterns": ("p_pattern", [("Numbers (e.g. 2,4,6)", "5,10,15")]),
        "Fraction Basics": ("fraction", [("Numerator", "1"), ("Denominator", "4")]),
        "Metric Convert": ("metric_convert", [("Value", "5000"), ("From (cm/m/km/g/kg)", "g"), ("To", "kg")]),
    }

    @staticmethod
    def number_to_words(n):
        units = ["Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"]
        n = int(n)
        if 0 <= n < 10: return {"result": units[n]}
        return {"result": str(n), "note": "Primary focus"}

    @staticmethod
    def tables(n, up):
        n, up = int(n), int(up)
        res = [f"{n} x {i} = {n*i}" for i in range(1, up+1)]
        return {"Table": res}

    @staticmethod
    def area_peri(l, w):
        l, w = float(l), float(w)
        return {"Area": l * w, "Perimeter": 2 * (l + w)}

    @staticmethod
    def money_add(a1, a2):
        return {"Total": f"₹ {float(a1) + float(a2)}"}

    @staticmethod
    def p_pattern(seq_str):
        nums = [int(x.strip()) for x in seq_str.split(",")]
        if len(nums) < 2: return {"Error": "Need 2 numbers"}
        diff = nums[1] - nums[0]
        return {"Next": nums[-1] + diff, "Rule": f"Add {diff}"}

    @staticmethod
    def fraction(n, d):
        n, d = int(n), int(d)
        if d == 0: return {"Error": "Div by zero"}
        return {"Decimal": n/d, "Type": "Proper" if n < d else "Improper"}

    @staticmethod
    def metric_convert(value, from_unit, to_unit):
        u = {"kg":1000, "g":1, "m":1, "cm":0.01, "km":1000}
        f, t = from_unit.lower(), to_unit.lower()
        if f in u and t in u:
            res = float(value) * u[f] / u[t]
            return {"Result": f"{res} {t}"}
        return {"Error": "Invalid unit"}

# Registry
PRIMARY_MATHS_REGISTRY = {
    "Primary (1-5)": Maths_Primary
}

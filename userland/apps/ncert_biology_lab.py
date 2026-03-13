"""
SigmaOS NCERT Biology Lab v10.0 — The Ultimate Series
Classes 6–12 | Exhaustive NCERT Experiment & Simulation Hub
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Biology_Classes_6_10:
    TITLE = "Secondary Biology: Living Systems"
    EXP_DATA = {
        "Starch/Iodine": ("starch", [("Iodine", "1")]),
        "BMI Calculator": ("bmi", [("Weight (kg)", "70"), ("Height (m)", "1.75")]),
        "Microscope Mag": ("micro", [("Obj (X)", "10"), ("Eye (X)", "10")]),
        "Heart Zones": ("heart", [("Age", "15"), ("Post-Ex (1/0)", "0")]),
        "Photosynthesis": ("photo", [("Light (1-10)", "8"), ("CO2 (1-10)", "7")]),
        "Osmosis Flow": ("osmosis", [("Cell_C (M)", "0.1"), ("Sol_C (M)", "0.5")]),
        "Blood Group Type": ("blood", [("Antigen A (1/0)", "1"), ("Antigen B", "0"), ("Rh Factor", "1")]),
        "Plant Type": ("plant", [("Leaves", "Dicot"), ("Root", "Tap")]),
    }

    @staticmethod
    def starch(i):
        return {"Status": "Blue-Black (PRESENT)" if int(i) else "Brown (ABSENT)"}

    @staticmethod
    def bmi(w, h):
        b = w / h**2
        cat = "Underweight" if b<18.5 else "Normal" if b<25 else "Overweight"
        return {"BMI": _r(b, 1), "Category": cat}

    @staticmethod
    def micro(obj, eye):
        return {"Total Mag": obj * eye}

    @staticmethod
    def heart(age, ex):
        m = 220 - age
        rate = 72; range_ = "60-100"
        if ex: rate = 110; range_ = f"{int(m*0.6)}-{int(m*0.8)}"
        return {"Current bpm": rate, "Safety Range": range_}

    @staticmethod
    def photo(l, c):
        rate = (l * c) / 2
        return {"O2 Evolution": _r(rate, 1), "Efficiency": "High" if rate > 30 else "Normal"}

    @staticmethod
    def osmosis(cc, sc):
        if sc > cc: return {"Process": "Exosmosis", "Result": "Plasmolysis / Shrink"}
        if sc < cc: return {"Process": "Endosmosis", "Result": "Turgidity / Swell"}
        return {"Process": "Equilibrium", "Result": "No Change"}

    @staticmethod
    def blood(a, b, rh):
        g = "O"
        if a and b: g = "AB"
        elif a: g = "A"
        elif b: g = "B"
        return {"Group": g + ("+" if rh else "-")}

    @staticmethod
    def plant(l, r):
        if "dicot" in l.lower() or "tap" in r.lower(): return {"Type": "DICOT", "Seeds": "2 Cotyledons"}
        return {"Type": "MONOCOT", "Seeds": "1 Cotyledon"}

class Biology_Classes_11_12:
    TITLE = "Senior Biology: Research & Genetics"
    EXP_DATA = {
        "Mendel Dihybrid": ("mendel", [("P1 (RRYY)", "RrYy"), ("P2 (RRYY)", "RrYy")]),
        "Hardy-Weinberg": ("hardy", [("p-freq", "0.6"), ("Pop Total", "1000")]),
        "Trophic 10%": ("trophic", [("Producer (kcal)", "1000")]),
        "RQ Substrate": ("rq", [("CO2 Vol", "1"), ("O2 Vol", "1")]),
        "Peptide/DNA Comp": ("dna", [("Seq", "ATGCGTA")]),
        "Population Nt": ("pop", [("N0", "100"), ("r", "0.1"), ("t", "10")]),
        "Quadrat Density": ("quadrat", [("Counts (cm separated)", "5,8,4,12"), ("Area (sq m)", "1")]),
        "Transpire Rate": ("transpire", [("Wind", "5"), ("Humidity", "2")]),
        "Mitosis Phases": ("mitosis", [("Phase", "Metaphase")]),
    }

    @staticmethod
    def mendel(p1, p2):
        # simplified 9:3:3:1
        return {"Phenotype Ratio": "9:3:3:1 (RoundYellow, RoundGreen, WrinkledYellow, WrinkledGreen)"}

    @staticmethod
    def hardy(p, n):
        q = 1 - p
        return {"AA": int(n*p**2), "Aa": int(n*2*p*q), "aa": int(n*q**2)}

    @staticmethod
    def trophic(e):
        return {"Herbivore": e*0.1, "Carnivore": e*0.01, "TopPredator": e*0.001}

    @staticmethod
    def rq(c, o):
        r = c/o
        return {"RQ": _r(r, 2), "Sub": "Carb" if 0.95<r<1.05 else "Fat/Protein"}

    @staticmethod
    def dna(s):
        d = {"A":"T", "T":"A", "C":"G", "G":"C"}
        return {"Complement": "".join(d.get(b.upper(), b) for b in s)}

    @staticmethod
    def pop(n0, r, t):
        nt = n0 * math.exp(r*t)
        return {"Final Pop": _r(nt, 0)}

    @staticmethod
    def quadrat(c, a):
        v = [int(x) for x in c.split(",")]
        return {"Avg Density": _r(sum(v)/(len(v)*a), 2)}

    @staticmethod
    def transpire(w, h):
        rate = (w * 5) / h
        return {"Rate": _r(rate, 2), "Status": "Intense" if rate > 20 else "Mild"}

    @staticmethod
    def mitosis(p):
        d = {"prophase": "Nucleus thickens, membrane fades", "metaphase": "Chromosomes line up at center", "anaphase": "Chromatids separate"}
        return {"Observation": d.get(p.lower(), "Nuclear division stage")}

BIOLOGY_REGISTRY = {
    "Classes 6-10": Biology_Classes_6_10,
    "Classes 11-12": Biology_Classes_11_12,
}

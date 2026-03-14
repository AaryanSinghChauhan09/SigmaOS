"""
SigmaOS NCERT Biology Lab v10.1 — The Ultimate Series
Classes 6–12 | Exhaustive NCERT Experiment & Simulation Hub
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Biology_Classes_6_10:
    TITLE = "Secondary Biology: Advanced Life Systems"
    EXP_DATA = {
        "Starch/Iodine": ("starch", [("Iodine", "1")]),
        "BMI Calculator": ("bmi", [("Weight (kg)", "70"), ("Height (m)", "1.75")]),
        "Microscope Mag": ("micro", [("Obj (X)", "10"), ("Eye (X)", "10")]),
        "Heart Zones": ("heart", [("Age", "15"), ("Post-Ex (1/0)", "0")]),
        "Photosynthesis": ("photo", [("Light (1-10)", "8"), ("CO2 (1-10)", "7")]),
        "Osmosis Flow": ("osmosis", [("Cell_C (M)", "0.1"), ("Sol_C (M)", "0.5")]),
        "Blood Group Type": ("blood", [("Antigen A (1/0)", "1"), ("Antigen B", "0"), ("Rh Factor", "1")]),
        "Plant Anatomy": ("plant", [("Leaves", "Dicot"), ("Root", "Tap")]),
        "Digestion Rate": ("digestion", [("pH", "2"), ("Temp (C)", "37")]),
    }

    @staticmethod
    def starch(i):
        return {"Status": "Blue-Black" if int(i) else "Brown"}

    @staticmethod
    def bmi(w, h):
        b = w / h**2
        return {"BMI": _r(b, 1), "Cat": "Normal" if 18.5<=b<25 else "Other"}

    @staticmethod
    def micro(obj, eye):
        return {"Mag": obj * eye}

    @staticmethod
    def heart(age, ex):
        m = 220 - age
        return {"bpm": 110 if ex else 72, "Max": m}

    @staticmethod
    def photo(l, c):
        rate = (l * c) / 2
        return {"Rate": _r(rate, 1)}

    @staticmethod
    def osmosis(cc, sc):
        if sc > cc: return {"Process": "Exosmosis"}
        if sc < cc: return {"Process": "Endosmosis"}
        return {"Process": "Equil"}

    @staticmethod
    def blood(a, b, rh):
        g = "O"
        if a and b: g = "AB"
        elif a: g = "A"
        elif b: g = "B"
        return {"Group": g + ("+" if rh else "-")}

    @staticmethod
    def plant(l, r):
        if "dicot" in l.lower() or "tap" in r.lower(): return {"Type": "DICOT"}
        return {"Type": "MONOCOT"}

    @staticmethod
    def digestion(ph, t):
        if 1.5 < ph < 2.5 and 35 < t < 40: return {"Status": "ACTIVE (Gastric)"}
        return {"Status": "INACTIVE"}

class Biology_Classes_11_12:
    TITLE = "Senior Biology: Research & Advanced Logic"
    EXP_DATA = {
        "Mendel Dihybrid": ("mendel", [("P1", "RrYy")]),
        "Hardy-Weinberg": ("hardy", [("p-freq", "0.6"), ("Pop", "1000")]),
        "RQ Substrate": ("rq", [("CO2 Vol", "1"), ("O2 Vol", "1")]),
        "DNA Complement": ("dna", [("Seq", "ATGCGTA")]),
        "Population Growth": ("pop", [("N0", "100"), ("r", "0.1"), ("t", "10")]),
        "Quadrat Density": ("quadrat", [("Counts", "5,8,4"), ("Area", "1")]),
        "Transpiration": ("transpire", [("Wind", "5"), ("Humidity", "2")]),
        "Mitosis Phases": ("mitosis", [("Phase", "Metaphase")]),
        "Lung Capacities": ("lung", [("TV (ml)", "500"), ("IRV (ml)", "3000"), ("ERV (ml)", "1100")]),
        "Pollen Lab": ("pollen", [("Sugar (%)", "10"), ("Time (min)", "30")]),
        "Pedigree Chart": ("pedigree", [("Father", "Affected"), ("Mother", "Carrier")]),
        "Water BOD": ("bod", [("DO Day 0", "9"), ("DO Day 5", "4")]),
        "Respiration Seeds": ("respiration", [("Time (hr)", "24"), ("CO2 Vol", "5")]),
        "Plasmolysis": ("plasmolysis", [("Sugar (%)", "20"), ("Cell Type", "Rhoeo")]),
        "DNA Isolation": ("dna_iso", [("Ethanol (C)", "-20"), ("Sample", "Papaya")]),
    }

    @staticmethod
    def mendel(p):
        return {"Ratio": "9:3:3:1 (Mendelian Dihybrid)"}

    @staticmethod
    def hardy(p, n):
        q = 1 - p
        return {"AA": int(n*p**2), "Aa": int(n*2*p*q), "aa": int(n*q**2)}

    @staticmethod
    def rq(c, o):
        r = c/o
        return {"RQ": _r(r, 2), "Sub": "Carb" if 0.95<r<1.05 else "Fat/Protein"}

    @staticmethod
    def dna(s):
        d = {"A":"T", "T":"A", "C":"G", "G":"C"}
        # Cast to str explicitly to satisfy linter joined string check
        res = [str(d.get(b.upper(), b)) for b in s]
        return {"Complement": "".join(res)}

    @staticmethod
    def pop(n0, r, t):
        return {"Final": _r(n0 * math.exp(r*t), 0)}

    @staticmethod
    def quadrat(c, a):
        v = [int(x) for x in str(c).split(",")]
        return {"Density": _r(sum(v)/(len(v)*a), 2)}

    @staticmethod
    def transpire(w, h):
        rate = (w * 5) / h
        return {"Rate": _r(rate, 2)}

    @staticmethod
    def mitosis(p):
        d = {"metaphase": "Aligned at Equator", "anaphase": "Separating"}
        return {"Obs": d.get(p.lower(), "Division Stage")}

    @staticmethod
    def lung(tv, irv, erv):
        vc = tv + irv + erv
        return {"Vital Capacity (ml)": vc, "IC (Inspiratory)": tv+irv}

    @staticmethod
    def pollen(s, t):
        if 5 < s < 15 and t > 20: return {"Status": "SUCCESSFUL GERMINATION", "Tube Length": "Long"}
        return {"Status": "FAILED", "Reason": "Sugar Mismatch"}

    @staticmethod
    def pedigree(f, m):
        f, m = f.lower(), m.lower()
        if "affected" in f and "carrier" in m: return {"Risk": "50% Affected", "Type": "Sex-Linked Possible"}
        if "affected" in f and "affected" in m: return {"Risk": "100% Affected", "Status": "Homozygous"}
        return {"Risk": "Calculating...", "Action": "Check Gen2"}

    @staticmethod
    def bod(do0, do5):
        bod_val = do0 - do5
        return {"BOD (mg/L)": bod_val, "Quality": "Clean" if bod_val < 3 else "Polluted"}

    @staticmethod
    def respiration(t, v):
        rq = v / t
        return {"Status": "CO2 Detected", "Rate": _r(rq, 2)}

    @staticmethod
    def plasmolysis(s, c):
        if s > 10: return {"Observation": "Hypertonic -> Plasmolysis", "Status": "Shrunken Protoplast"}
        return {"Observation": "Isotonic/Hypotonic", "Status": "Turgid"}

    @staticmethod
    def dna_iso(temp, s):
        if temp < 0: return {"Step": "Chilled Ethanol Added", "Result": "DNA Threads Precipitate"}
        return {"Step": "Ethanol Too Warm", "Result": "Degradation"}

BIOLOGY_REGISTRY = {
    "Classes 6-10": Biology_Classes_6_10,
    "Classes 11-12": Biology_Classes_11_12,
}

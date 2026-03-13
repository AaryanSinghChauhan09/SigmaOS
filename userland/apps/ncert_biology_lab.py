"""
SigmaOS NCERT Biology Lab v8.0 — The Interactive series
Classes 6–12 | Every Core NCERT Biological study & Ecology
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Biology_Classes_6_10:
    TITLE = "Secondary Biology: Anatomy & Functions"
    EXP_DATA = {
        "Starch Test": ("starch", [("Iodine Presence (1/0)", "1")]),
        "Plant Growth (Variable)": ("growth", [("Sun (1-10)", "8"), ("Water (1-10)", "7"), ("Soil (1-10)", "9")]),
        "Cheek Cell Study": ("cheek", [("Stain (Methylene Blue)", "1")]),
        "Human Heart Rate": ("heart", [("Age", "15"), ("Post-exercise (1/0)", "0")]),
        "Mendel Monohybrid": ("mendel_m", [("P1 (TT/Tt/tt)", "Tt"), ("P2 (TT/Tt/tt)", "Tt")]),
        "Osmosis (Potato/Raisins)": ("osmosis", [("Inside", "Sugar"), ("Outside", "Water")]),
        "Dicot vs Monocot": ("plant_type", [("Venation (Reticulate/Parallel)", "Reticulate")]),
        "Sex Determination": ("gender", [("Inherited (X/Y)", "Y")]),
    }

    @staticmethod
    def starch(i):
        if int(i): return {"Result": "Blue-black", "Note": "Photosynthesis products detected"}
        return {"Result": "Brown", "Note": "No starch detected"}

    @staticmethod
    def growth(s, w, f):
        s, w, f = float(s), float(w), float(f)
        score = (s + w + f) / 3
        if score > 8: return {"Rate": "OPTIMAL", "Observation": "Large healthy leaves, strong stem"}
        if score > 5: return {"Rate": "AVERAGE", "Observation": "Slow growth, pale green color"}
        return {"Rate": "STUNTED", "Observation": "Wilting, low biomass"}

    @staticmethod
    def cheek(stain):
        if int(stain): return {"Visual": "Blue nucleus visible", "Type": "Eukaryotic Animal Cell"}
        return {"Visual": "Unclear transparent blobs"}

    @staticmethod
    def heart(age, ex):
        age, ex = float(age), int(ex)
        base = 72
        if ex: base += 35
        return {"Estimated Pulse (bpm)": int(base), "Range": "60-110 bpm"}

    @staticmethod
    def mendel_m(p1, p2):
        g = [a+b for a in p1 for b in p2]
        g = ["".join(sorted(x)) for x in g]
        return {"Offspring": g, "Phenotype": "3 Tall : 1 Dwarf (Approx)"}

    @staticmethod
    def osmosis(ins, outs):
        ins, outs = ins.lower(), outs.lower()
        if "sugar" in ins and "water" in outs: return {"Process": "ENDOSMOSIS", "Result": "Cell Swells / Level Rises"}
        if "water" in ins and "sugar" in outs: return {"Process": "EXOSMOSIS", "Result": "Cell Shrinks / Level Drops"}
        return {"Process": "EQUILIBRIUM", "Result": "No Change"}

    @staticmethod
    def plant_type(v):
        if "retic" in v.lower(): return {"Type": "DICOT", "Root": "Tap Root", "Example": "Gram, Pea"}
        return {"Type": "MONOCOT", "Root": "Fibrous Root", "Example": "Wheat, Maize"}

    @staticmethod
    def gender(chr):
        if "Y" in chr.upper(): return {"Offspring": "MALE (XY)", "Determiner": "Paternal"}
        return {"Offspring": "FEMALE (XX)", "Determiner": "Maternal inheritance"}

class Biology_Classes_11_12:
    TITLE = "Senior Biology: Advanced Physiology"
    EXP_DATA = {
        "Meiosis Stages": ("meiosis", [("Stage (Pro/Meta/Ana/Telo)", "Metaphase I")]),
        "Mitosis Simulation": ("mitosis", [("Tissue", "Onion Root Tip")]),
        "Transpiration (Potometer)": ("transpire", [("Fan Speed (1-10)", "5"), ("Humidity (1-10)", "3")]),
        "RQ Calculation": ("rq", [("CO2 Vol", "1"), ("O2 Vol", "1")]),
        "Species Area Richness": ("species", [("Area (sq km)", "1000")]),
        "Hardy-Weinberg Freq": ("hardy", [("p allele", "0.6")]),
        "Population Growth": ("growth", [("N0 (Initial)", "100"), ("r rate", "0.1"), ("t time", "10")]),
        "DNA Extraction logic": ("dna", [("Reagent", "Cold Ethanol")]),
    }

    @staticmethod
    def meiosis(s):
        s = s.lower()
        if "meta" in s: return {"Occurrence": "Bivalent chromosomes align at equator"}
        if "ana" in s: return {"Occurrence": "Homologous chromosomes move to opposite poles"}
        return {"Note": "Reductional division occurring"}

    @staticmethod
    def mitosis(t):
        return {"Note": "Equational division. Separation of sister chromatids into daughter cells."}

    @staticmethod
    def transpire(v, h):
        v, h = float(v), float(h)
        rate = (v * 2) - (h / 2) + 5
        return {"Relative Rate": _r(rate, 2), "Inference": "Highest in Windy, Low Humidity environments"}

    @staticmethod
    def rq(co2, o2):
        res = float(co2) / float(o2)
        substrate = "Carbohydrate" if 0.95 < res <= 1.0 else ("Fat/Protein" if res < 1 else "Organic Acid")
        return {"RQ": _r(res, 2), "Likely Substrate": substrate}

    @staticmethod
    def species(a):
        s = 0.5 * (float(a)**0.3)
        return {"Richness S": _r(s, 2)}

    @staticmethod
    def hardy(p):
        p = float(p); q = 1.0 - p
        return {"AA (p²)": _r(p**2, 3), "Aa (2pq)": _r(2*p*q, 3), "aa (q²)": _r(q**2, 3)}

    @staticmethod
    def growth(n0, r, t):
        n0, r, t = float(n0), float(r), float(t)
        # Exponential growth: Nt = N0 * e^rt
        nt = n0 * math.exp(r * t)
        return {"Final Population Nt": _r(nt, 0)}

    @staticmethod
    def dna(r):
        if "ethanol" in r.lower(): return {"Observation": "DNA threads precipitate and become visible (Spooling possible)"}
        return {"Observation": "DNA remains in solution"}

BIOLOGY_REGISTRY = {
    "Classes 6-10": Biology_Classes_6_10,
    "Classes 11-12": Biology_Classes_11_12,
}

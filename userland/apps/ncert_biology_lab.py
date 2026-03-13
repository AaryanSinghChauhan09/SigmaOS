"""
SigmaOS NCERT Biology Lab v6.0 — The Ultimate Series
Classes 6–12 | Every Core NCERT Experiment & Ecosystem Simulation
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Biology_Classes_6_10:
    TITLE = "Secondary Biology: Life Processes, Control & Genetics"
    EXP_DATA = {
        "Starch Test": ("starch", [("Iodine (1=Yes, 0=No)", "1")]),
        "Cell Organelles": ("cell", [("Organelle", "Nucleus")]),
        "Mendel's Law": ("mendel", [("P1 (TT/Tt/tt)", "Tt"), ("P2 (TT/Tt/tt)", "Tt")]),
        "Human Digestion": ("digest", [("Enzyme (Amylase/Pepsin/Lipase)", "Amylase")]),
        "Pulse Rate": ("heart", [("Age", "15"), ("Post-exercise (1/0)", "0")]),
        "Blood Donor Match": ("blood", [("Donor", "O-"), ("Recipient", "A+")]),
        "Tropism (Plants)": ("tropism", [("Type (Hydro/Photo/Geo)", "Photo")]),
        "Ecosystem Roles": ("eco", [("Organism", "Lion")]),
        "Sex Determination": ("gender", [("Inherited from Father (X/Y)", "Y")]),
    }

    @staticmethod
    def starch(i):
        if int(i): return {"Result": "Blue-black Color", "Inference": "Starch Present (Photosynthesis verified)"}
        return {"Result": "No color change", "Inference": "No Starch found"}

    @staticmethod
    def cell(n):
        data = {"nucleus": "Control center, contains DNA", "mitochondria": "Powerhouse, ATP production", "ribosome": "Protein synthesis"}
        return {"Function": data.get(n.lower(), "Refer NCERT Science Class 9")}

    @staticmethod
    def mendel(p1, p2):
        g = [a+b for a in p1 for b in p2]
        g = ["".join(sorted(x)) for x in g]
        return {"Offspring Genotypes": g, "Phenotype Ratio": "Approx 3:1 (Dominant:Recessive)"}

    @staticmethod
    def digest(e):
        data = {"amylase": "Starch -> Sugar", "pepsin": "Proteins -> Peptides", "lipase": "Emulsified Fats -> Fatty acids"}
        return {"Catalysis": data.get(e.lower(), "Refer Class 10 Biology")}

    @staticmethod
    def heart(age, ex):
        age, ex = float(age), int(ex)
        base = 72
        if ex: base += 35
        return {"Pulse (bpm)": base, "Note": "Standard average is 72 bpm"}

    @staticmethod
    def blood(d, r):
        d, r = d.upper(), r.upper()
        if "O-" in d or d == r or "AB+" in r: return {"Outcome": "COMPATIBLE"}
        return {"Outcome": "INCOMPATIBLE / RISKY"}

    @staticmethod
    def tropism(t):
        data = {"photo": "Growth towards light", "hydro": "Growth towards water", "geo": "Growth towards gravity"}
        return {"Response": data.get(t.lower(), "General Plant Motion")}

    @staticmethod
    def eco(o):
        data = {"lion": "Carnivore / Top Predator", "cow": "Herbivore / Primary Consumer", "plant": "Producer"}
        return {"Niche": data.get(o.lower(), "Consumer")}

    @staticmethod
    def gender(chr):
        if "Y" in chr.upper(): return {"Offspring": "Male (XY)"}
        return {"Offspring": "Female (XX)"}

class Biology_Classes_11_12:
    TITLE = "Senior Biology: Physiology, Ecology, Biotech & Evolution"
    EXP_DATA = {
        "Species Area Curves": ("species", [("Area (sq km)", "1000")]),
        "Hardy-Weinberg": ("hardy", [("p freq", "0.6")]),
        "PCR Yield": ("pcr", [("Initial mols", "1"), ("Cycles", "30")]),
        "Respiratory Quotient": ("rq", [("CO2 vol", "102"), ("O2 vol", "145")]),
        "Trophic Efficiency": ("trophic", [("Producer Energy (J)", "10000")]),
        "DNA Conversion": ("dna", [("Strand (ATGC)", "ATGC")]),
        "Osmosis (Turgidity)": ("osmosis", [("Soln (Hypo/Hyper/Iso)", "Hyper")]),
        "Diversity Index": ("diversity", [("Counts (comma-sep)", "10,20,5,5")]),
    }

    @staticmethod
    def species(a):
        # S = CA^Z (C=0.5, Z=0.3)
        s = 0.5 * (float(a)**0.3)
        return {"Species Richness S": _r(s, 2)}

    @staticmethod
    def hardy(p):
        p = float(p); q = 1.0 - p
        return {"AA (p²)": _r(p**2, 4), "Aa (2pq)": _r(2*p*q, 4), "aa (q²)": _r(q**2, 4)}

    @staticmethod
    def pcr(n, c):
        return {"Resulting Copies": int(float(n) * (2**float(c)))}

    @staticmethod
    def rq(co2, o2):
        res = float(co2)/float(o2)
        return {"RQ": _r(res, 2), "Substrate": "Fat/Protein" if res < 1 else "Carbohydrate"}

    @staticmethod
    def trophic(e):
        e = float(e)
        return {"Primary Consumer": e*0.1, "Secondary": e*0.01, "Tertiary": e*0.001}

    @staticmethod
    def dna(s):
        comp = {"A":"T", "T":"A", "G":"C", "C":"G"}
        return {"Complementary DNA": "".join(comp.get(x.upper(), "?") for x in s)}

    @staticmethod
    def osmosis(type):
        t = type.lower()
        if "hyper" in t: return {"Effect": "Exosmosis", "Result": "Plasmolysis / Cell shrinks"}
        if "hypo" in t: return {"Effect": "Endosmosis", "Result": "Turgidity / Cell swells"}
        return {"Effect": "Equilibrium", "Result": "Flaccid"}

    @staticmethod
    def diversity(c_str):
        counts = [float(x.strip()) for x in c_str.split(",")]
        total = sum(counts)
        # Simplified Simpson-like: D = sum( (n/N)^2 )
        d = sum((n/total)**2 for n in counts)
        return {"Index D": _r(d, 3), "Note": "Lower D = Higher Diversity"}

BIOLOGY_REGISTRY = {
    "Classes 6-10": Biology_Classes_6_10,
    "Classes 11-12": Biology_Classes_11_12,
}

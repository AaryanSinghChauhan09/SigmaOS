"""
SigmaOS NCERT Biology Lab v5.0 — The Complete Series
Classes 6–12 | Comprehensive Bio labs & ecology simulations
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Biology_Classes_6_10:
    TITLE = "Classes 6–10: Cell Biology, Life Processes & Genetics"
    EXP_DATA = {
        "Starch Test (Photosynthesis)": ("starch", [("Iodine (1/0)", "1")]),
        "Cell Size Ratio": ("cell_size", [("Radius (um)", "10")]),
        "Mendel Monohybrid": ("mendel_m", [("P1 (TT/Tt/tt)", "Tt"), ("P2 (TT/Tt/tt)", "Tt")]),
        "Mendel Dihybrid": ("mendel_d", [("P1 (RRYY)", "RRYY"), ("P2 (rryy)", "rryy")]),
        "Blood Grouping": ("blood", [("Donor", "A+"), ("Recipient", "O+")]),
        "Digestive Enzymes": ("digest", [("Organ", "Stomach")]),
        "Reflex Action Path": ("reflex", [("Stimulus", "Heat")]),
    }

    @staticmethod
    def starch(i):
        if int(i): return {"Result": "Blue-black", "Note": "Starch found"}
        return {"Result": "Brown", "Note": "Starch absent"}

    @staticmethod
    def cell_size(r):
        area = 4 * math.pi * r**2
        vol = (4/3) * math.pi * r**3
        return {"S.Area": _r(area, 1), "Volume": _r(vol, 1), "S/V Ratio": _r(area/vol, 3)}

    @staticmethod
    def mendel_m(p1, p2):
        g = [a+b for a in p1 for b in p2]
        g = ["".join(sorted(x)) for x in g]
        return {"Genotypes": g, "Phenotype Ratio": "3 Dominant : 1 Recessive (Approx)"}

    @staticmethod
    def mendel_d(p1, p2):
        # Very simplified representation
        return {"F1 Generation": "RrYy (All)", "F2 Pheno Ratio": "9:3:3:1"}

    @staticmethod
    def blood(d, r):
        # Simplified
        if "O-" in d or d == r: return {"Status": "COMPATIBLE"}
        return {"Status": "RISKY / INCOMPATIBLE"}

    @staticmethod
    def digest(org):
        d = {"mouth": "Amylase", "stomach": "Pepsin, HCl", "pancreas": "Lipase, Trypsin"}
        return {"Active Agents": d.get(org.lower(), "Refer NCERT")}

    @staticmethod
    def reflex(s):
        return {"Path": "Receptor -> Sensory Nerve -> Spinal Cord -> Motor Nerve -> Muscle"}

class Biology_Classes_11_12:
    TITLE = "Classes 11–12: Physiology, Biotechnology & Ecology"
    EXP_DATA = {
        "Species Area Curves": ("species", [("Area (sq km)", "1000")]),
        "Hardy-Weinberg Freq": ("hardy", [("p freq", "0.6")]),
        "PCR Copies yield": ("pcr", [("Init mols", "1"), ("Cycles", "30")]),
        "RQ (Respiration)": ("rq", [("CO2 vol", "1"), ("O2 vol", "1")]),
        "DNA base pairing": ("dna", [("Strand", "ATGC")]),
        "Population growth": ("pop", [("No", "100"), ("r rate", "0.1"), ("t time", "10")]),
        "Transpiration Rate": ("transpire", [("Air Speed (m/s)", "2"), ("Humidity (%)", "50")]),
    }

    @staticmethod
    def species(a):
        s = 0.5 * (a**0.3)
        return {"Richness S": _r(s, 2)}

    @staticmethod
    def hardy(p):
        q = 1-p
        return {"p2 (AA)": _r(p**2, 4), "2pq (Aa)": _r(2*p*q, 4), "q2 (aa)": _r(q**2, 4)}

    @staticmethod
    def pcr(n, c):
        return {"Yield": int(n * (2**c))}

    @staticmethod
    def rq(co2, o2):
        return {"RQ": _r(co2/o2, 2)}

    @staticmethod
    def dna(s):
        comp = {"A":"T", "T":"A", "G":"C", "C":"G"}
        return {"Complementary": "".join(comp.get(x.upper(), "?") for x in s)}

    @staticmethod
    def pop(n0, r, t):
        nt = n0 * math.exp(r*t)
        return {"Pop at T": _r(nt, 0)}

    @staticmethod
    def transpire(v, h):
        rate = (v * 10) / (h / 10)
        return {"Relative Rate": _r(rate, 2), "Observation": "Increases with Wind, Decreases with Humidity"}

BIOLOGY_REGISTRY = {
    "Classes 6-10": Biology_Classes_6_10,
    "Classes 11-12": Biology_Classes_11_12,
}
